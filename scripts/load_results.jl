using JSON
using OrderedCollections
using AirspeedVelocity

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


MT_BRANCH = "ale/3.0"

air = AirspeedVelocity.load_results(
    "Metatheory", [MT_BRANCH],
    input_dir="/Users/niklas/.julia/dev/Metatheory/results"
)

results = OrderedDict(
    "egg" => load_results(joinpath(".", "target", "criterion")),
    "Metatheory" => air[MT_BRANCH],
)

new_res = OrderedDict(
    rev => OrderedDict(
         replace(k, "/"=>"_") => v for (k,v) in d
    ) for (rev, d) in results
)

AirspeedVelocity.create_table(new_res) |> print
