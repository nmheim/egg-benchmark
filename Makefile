BRANCH1 := ale/3.0
BRANCH2 := master
MT_RESULTS_DIR := ./target/mt_results

.PHONY: mt-bench egg-bench results-table

mt-bench:
	# make sure that necessary packages are installed
	julia --project=scripts -e 'using Pkg; Pkg.instantiate()'
	# run benchmarks
	@if [ ! -d "$(MT_RESULTS_DIR)" ]; then mkdir -p $(MT_RESULTS_DIR); fi
	$(HOME)/.julia/bin/benchpkg Metatheory \
		-r $(BRANCH1),$(BRANCH2) \
		--bench-on=$(BRANCH1) \
		--output-dir=$(MT_RESULTS_DIR)

egg-bench:
	cargo bench

results-table:
	# TODO: don't cut off long table header names
	julia --project=scripts scripts/load_results.jl \
		--mt-results=$(MT_RESULTS_DIR) \
		-b $(BRANCH1) -b $(BRANCH2)
