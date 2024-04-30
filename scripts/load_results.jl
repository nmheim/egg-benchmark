using JSON
using OrderedCollections
using AirspeedVelocity
using Printf: @sprintf
using ArgParse

s = ArgParseSettings()
@add_arg_table s begin
    "--run"
        help = "Run the benchmarks"
        action = :store_true
    "--with-confidence", "-c"
        help = "Print confidence intervals in table."
        action = :store_true
end
parsed_args = parse_args(ARGS, s)
MT_30 = "ale/3.0"
MT_20 = "master"
MT_RESULTS_DIR = joinpath(pwd(), "target", "Metatheory")

function load_results(path::String)
    (_, dirs, _) = walkdir(path) |> first

    # remove report directory
    dirs = filter(d -> d!="report", dirs)

    # read results
    benchpaths = map(d -> joinpath(path, d, "new", "estimates.json"), dirs)
    crit_results = OrderedDict(bench => JSON.parsefile(path) for (bench, path) in zip(dirs, benchpaths))

    z75 = 1.15
    z95 = 1.96

    # output point estimates
    OrderedDict(
        bench => Dict(
            "median"=> d["median"]["point_estimate"],
            "mean"  => d["mean"]["point_estimate"],
            "std"   => d["std_dev"]["point_estimate"],
            "75"    => d["median"]["confidence_interval"]["upper_bound"] * z75/z95,
            "25"    => d["median"]["confidence_interval"]["lower_bound"] * z75/z95,
        )
        for (bench, d) in crit_results
    )
end

function format_val(val::Dict; confidence_interval=true)
    if haskey(val, "75") && confidence_interval
        unit, unit_name = val["median"]>1e6 ? (1e-6, "ms") : (1e-3, "μs")
        @sprintf(
            "%.3f ± %.2f %s",
            val["median"] * unit,
            (val["75"] - val["25"]) * unit,
            unit_name
        )
    elseif haskey(val, "median")
        unit, unit_name = val["median"]>1e6 ? (1e-6, "ms") : (1e-3, "μs")
        @sprintf("%.3g %s", val["median"] * unit, unit_name)
    else
        @sprintf("%.3g", val["speedup"])
    end
end
format_val(::Missing; kw...) = @sprintf("")

function ratio_column!(combined_results, c1, c2, key="median")
    all_keys = combined_results[c1] |> keys
    col = OrderedDict{String,Dict}()
    for row in all_keys
        if haskey(combined_results[c2], row)
            a = combined_results[c1][row][key]
            b = combined_results[c2][row][key]
            ratio = a/b
            col[row] = Dict("speedup"=>a/b)
        end
    end

    combined_results["$c1/$c2"] = col
    combined_results
end


if parsed_args["run"]
    # run egg benches
    run(`cargo bench`)

    # run benchpkg on Metatheory
    isdir(MT_RESULTS_DIR) || mkdir(MT_RESULTS_DIR)
    run(`$(homedir())/.julia/bin/benchpkg Metatheory -r $MT_30,$MT_20 --bench-on=$MT_30 --output-dir=$MT_RESULTS_DIR`)
end

air = AirspeedVelocity.load_results(
    "Metatheory", [MT_30, MT_20],
    input_dir="/Users/niklas/.julia/dev/Metatheory/results"
)

egg = load_results(joinpath(".", "target", "criterion"))

egg_customlang = Dict(k=>v for (k,v) in egg if occursin("customlang", k))
egg_symbollang = Dict(k=>v for (k,v) in egg if k ∉ keys(egg_customlang))
egg_customlang = Dict(replace(k, "customlang_"=>"")=>v for (k,v) in egg_customlang)
results = OrderedDict(
    "egg-sym" => egg_symbollang,
    "egg-cust" => egg_customlang,
    "MT@2.0" => air[MT_20],
    "MT@3.0" => air[MT_30],
)

new_res = OrderedDict(
    rev => OrderedDict(
         replace(k, "/"=>"_") => v for (k,v) in d
    ) for (rev, d) in results
)


ratio_column!(new_res, "egg-sym", "MT@3.0")
ratio_column!(new_res, "egg-cust", "MT@3.0")
ratio_column!(new_res, "MT@2.0", "MT@3.0")
AirspeedVelocity.create_table(
    new_res, formatter=v->format_val(v;confidence_interval=parsed_args["with-confidence"])) |> print
