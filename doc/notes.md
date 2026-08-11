
# Documentation & API tasks:

+ Write tests for single-column tables.
+ Write API documentation


# Implementation tasks:

+ Support full table width constraints
+ Implement colors for terminal renderers
+ Implement renderer formatting hooks
+ Support custom line breaker
+ Support extra width in aggregator
+ Deduplicate column references
+ Support independent header/footer alignment
+ Support unicode string widths
+ Implement Features
+ Implement parallelism
+ Implement custom aggregation
+ Support subcolumns
+ Support multiple header/footer rows
+ Implement more robust formatting options

# Deferred tasks:

+ Deduplicate sort indices
	- Requires copying the order
	- little payoff
+ Implement Transpose operation
	- Can no longer expose ColumnDesc array to renderers.
	- Need a way to request column descriptors dynamically.
