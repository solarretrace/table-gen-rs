
# Documentation & API tasks:

+ Write API documentation

# Implementation tasks:

+ Support outer border elision in terminal renderers
+ Support independent header/footer alignment
+ Implement parallelism
+ Implement custom aggregation
+ Support subcolumns
	- Splice adapter
	- Support multiple header/footer rows
	- Support joined column rendering
+ Implement more robust formatting options

# Deferred tasks:

+ Deduplicate sort indices
	- Requires copying the order
	- little payoff
+ Implement Transpose operation
	- Can no longer expose ColumnDesc array to renderers.
	- Need a way to request column descriptors dynamically.
+ Deduplicate column references
	- Unnecessary. Duplicate columns can have different formatting once features are implemented.
+ Forced column narrowing
	- Unnecessary. You could just remove min widths from ColumnDefs.

