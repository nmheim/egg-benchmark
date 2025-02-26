using JSON
using OrderedCollections
using AirspeedVelocity
using Printf: @sprintf
using ArgParse

s = ArgParseSettings()
@add_arg_table s begin
    "--with-confidence", "-c"
        help = "Print confidence intervals in table."
        action = :store_true
    "--mt-results"
        help = "Results directory of Metatheory benchmark."
        arg_type = String
    "--egg-results"
        help = "Results directory of egg benchmark."
        arg_type = String
        default = joinpath(".", "target", "criterion")
    "--branches", "-b"
        help = "Branches to benchmark. Pass multiple with: -b BRANCH1 -b BRANCH2 ..."
        arg_type = String
        action = :append_arg
    "--output", "-o"
        help = "File to store the table"
        arg_type = String
end
parsed_args = parse_args(ARGS, s)
MT_RESULTS_DIR = parsed_args["mt-results"]
EGG_RESULTS_DIR = parsed_args["egg-results"]
BRANCHES = parsed_args["branches"]
OUTPUT = parsed_args["output"]

display(parsed_args)

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
    od = OrderedDict(
        bench => Dict(
            "median"=> d["median"]["point_estimate"],
            "mean"  => d["mean"]["point_estimate"],
            "std"   => d["std_dev"]["point_estimate"],
            "75"    => d["median"]["confidence_interval"]["upper_bound"] * z75/z95,
            "25"    => d["median"]["confidence_interval"]["lower_bound"] * z75/z95,
        )
        for (bench, d) in crit_results
    )
    
    
    # parse egraph size from logs
    pat = r"\[[^\]]+] (?<bench>.+) n_classes: (?<n_classes>\d+), n_nodes: (?<n_nodes>\d+), n_memo: (?<n_memo>\d+)"
    open("./target/egg-log.txt", "r") do io
        d = Dict()
        for line in eachline(io)
            m = match(pat, line)
            !isnothing(m) || continue
            bench = replace(m["bench"], "/" => "_")
            n_classes = parse(Int,m["n_classes"])
            n_nodes = parse(Int, m["n_nodes"])
            n_memo = parse(Int, m["n_memo"])
            
            (v1,v2,v3) = get!(d, bench, (UInt64[],UInt64[],UInt64[]))
            push!(v1, n_classes); push!(v2, n_nodes); push!(v3, n_memo)
        end
        
        avg(xs) = sum(xs) / length(xs)
        for (bench, vectors) in d
            push!(od[bench], "n_classes_avg" => avg(vectors[1]))
            push!(od[bench], "n_nodes_avg" => avg(vectors[2]))
            push!(od[bench], "n_memo_avg" => avg(vectors[3]))
        end
    end
    od
end

function load_size_results!(od)
    # parse egraph size from logs
    pat1 = r".*Info.*Running benchmarks for [^@]+@(?<branch>[^:]+):"
    # [ Info: Running benchmarks for Metatheory@31db7e8:
    pat2 = r"(?<bench>.+) n_classes: (?<n_classes>\d+), n_nodes: (?<n_nodes>\d+), n_memo: (?<n_memo>\d+)"
    open("./target/mt_results/mt-log.txt", "r") do io
        curbranch = ""
        avg(xs) = sum(xs) / length(xs)
        for line in eachline(io)
            m1 = match(pat1, line)
            !isnothing(m1) && (curbranch = m1["branch"])
            
            m2 = match(pat2, line)
            !isnothing(m2) || continue
            
            bench = m2["bench"]
            n_classes = parse(Int,m2["n_classes"])
            n_nodes = parse(Int, m2["n_nodes"])
            n_memo = parse(Int, m2["n_memo"])

            push!(od[curbranch][bench], "n_classes_avg" => n_classes)
            push!(od[curbranch][bench], "n_nodes_avg" => n_nodes)
            push!(od[curbranch][bench], "n_memo_avg" => n_memo)
        end
        
    end
    od
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
    elseif haskey(val, "median ratio")
        @sprintf("%.3f", val["median ratio"])
    else
        ""
    end
end
format_val(::Missing; kw...) = @sprintf("")


function format_size(val::Dict)
    if haskey(val, "n_classes_avg")
        @sprintf("%i %i %i", val["n_classes_avg"], val["n_nodes_avg"], val["n_memo_avg"])
    elseif haskey(val, "n_classes_avg ratio")
        @sprintf("%.3f %.3f", val["n_classes_avg ratio"], val["n_nodes_avg ratio"])
    else
        ""
    end
end
format_size(::Missing; kw...) = @sprintf("")

function ratio_column!(combined_results, c1, c2, ratiokeys...)
    all_keys = combined_results[c1] |> keys
    col = OrderedDict{String,Dict}()
    for row in all_keys
        if haskey(combined_results[c2], row)
            for rkey in ratiokeys
                a = get(combined_results[c1][row], rkey, nothing)
                b = get(combined_results[c2][row], rkey, nothing)
                !isnothing(a) && !isnothing(b) || continue
                
                get!(col, row, Dict())["$rkey ratio"] = a/b
            end
        end
    end

    combined_results["$c1/$c2"] = col
    combined_results
end


air = AirspeedVelocity.load_results(
    "Metatheory", BRANCHES,
    input_dir=MT_RESULTS_DIR
)
load_size_results!(air)

egg = load_results(EGG_RESULTS_DIR)

egg_customlang = Dict(k=>v for (k,v) in egg if occursin("customlang", k))
egg_symbollang = Dict(k=>v for (k,v) in egg if k ∉ keys(egg_customlang))
egg_customlang = Dict(replace(k, "customlang_" => "")=>v for (k,v) in egg_customlang)
results = OrderedDict(
    "egg-sym" => egg_symbollang,
    "egg-cust" => egg_customlang,
)

for br in BRANCHES
    results["MT@$br"] = air[br]
end

new_res = OrderedDict(
    rev => OrderedDict(
         replace(k, "/" => "_") => v for (k,v) in d
    ) for (rev, d) in results
)


ratiokeys = ("median", "n_classes_avg", "n_nodes_avg")
ratio_column!(new_res, "egg-sym", "MT@$(BRANCHES[1])", ratiokeys...)
ratio_column!(new_res, "egg-cust", "MT@$(BRANCHES[1])", ratiokeys...)
for b2 in BRANCHES[2:end]
    ratio_column!(new_res, "MT@$b2", "MT@$(BRANCHES[1])", ratiokeys...)
end
table = AirspeedVelocity.create_table(
    new_res,
    formatter=v->format_val(v;confidence_interval=parsed_args["with-confidence"])
)

if !isnothing(OUTPUT)
    @info "Saving table at $(OUTPUT)"
    open(OUTPUT, "w") do io
        write(io, table)
    end
end
println(table)

# append another table with size information
table = AirspeedVelocity.create_table(
    new_res,
    formatter=v->format_size(v)
)

if !isnothing(OUTPUT)
    @info "Saving table at $(OUTPUT)"
    open(OUTPUT, "a") do io
        println(io)
        write(io, table)
    end
end
println(table)
