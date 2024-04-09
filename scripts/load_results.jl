using JSON
using OrderedCollections
using AirspeedVelocity

function load_results(path::String)
    (_, dirs, _) = walkdir(path) |> first

    # remove report directory
    dirs = filter(d -> d!="report", dirs)

    # read results
    benchpaths = map(d -> joinpath(path, d, "new", "estimates.json"), dirs)
    crit_results = Dict(bench => JSON.parsefile(path) for (bench, path) in zip(dirs, benchpaths))

    # v = crit_results |> values |> first
    # v["median"] |> display
    # v["mean"]["confidence_interval"] |> display

    # output point estimates
    Dict(
        bench => Dict(
            "median"=> d["median"]["point_estimate"],
            "mean"  => d["mean"]["point_estimate"],
            "std"   => d["std_dev"]["point_estimate"]
        )
        for (bench, d) in crit_results
    )
end


results = AirspeedVelocity.load_results(
    "Metatheory", ["nh/benchmark"],
    input_dir="/Users/niklas/.julia/dev/Metatheory/results"
)
results["egg"] = load_results(joinpath(".", "target", "criterion"))

new_res = OrderedDict(
    rev => OrderedDict(
         replace(k, "/"=>"_") => v for (k,v) in d
    ) for (rev, d) in air_res
)

AirspeedVelocity.create_table(new_res) |> print
