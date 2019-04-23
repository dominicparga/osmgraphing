# osmgraphing

Goal of this student project is parsing [openstreetmap][web_openstreetmap] data to calculate traffic routes on it.

## Goal

Long story short, the pipeline will look like this:

1. Download `raw osm data`
2. Read in `raw osm data` (probably partially)
3. Filter tags
4. Merge and process `osm-nodes` and `osm-edges`
5. Create `streetgraph`

Following requirements should be fulfilled:

- Machines __up to 16 GB RAM__ should be able run the project.
- Preprocessing should take __less than 10 minutes__.
- A __routing query__ should be processed in __under 10 seconds__.
- If __Contraction Hierarchies__ are used, the __speedup__ should be __up to 1000x__.
- The project should support at least __Ubuntu 18.04__.
- __Test maps__ are `Isle of Men`, `Stuttgart` or `Baden-Württemberg` and `Germany`.

### Download `raw osm data`

Downloaded osm data is provided in xml, where nodes are related to location in latitude and longitude.

Problems will be the size limit of downloading from [openstreetmap][web_openstreetmap], but there are other osm data providers like [geofabrik][web_geofabrik] for instance.

### Read `raw osm data`

Since osm data is stored in huge xml files (up to GB), reading in those files to parse them could be tricky.

### Filter tags

For routing, traffic crossroads are needed and many tags of the provided data is not needed.
So first step will be providing osm data and filtering unnecessary tags in a parsing step.

### Merge and process `osm-nodes` and `osm-edges`

To do routing on the remaining data, the parsing result has to be stored efficiently in a graph structure.
Since nodes are location related, not traffic-logic related, osm-nodes or osm-edges have to be merged into crossroads and edges, so routing algorithms don't have to execute useless steps.

### Create streetgraph

An offset-graph will fulfill the needs of quick node accesses.
Following picture shows a small example of the data structure.

```text
1 --> 2 <---+
|     |     |
+---> 3 <-- 4

nodes           1 2 3 4
edges per node  [(1 2) (1 3)] [(2 3)] [(3 2)] [(4 2) (4 3)]
offset          [0] [4] [6] [8]
```

A metric for calculating lengths is needes as well.
`Haversine distance` is the distance (e.g. in meters) between two points on a sphere (given in latitude and longitude).

## TODO

- Check `leaflet` - osm data provider?
- Write info about filtered tags (count them explicitly)
- Check order of latitude and longitude since changing it during development is quite annoying ._.

[web_openstreetmap]: https://openstreetmap.org
[web_geofabrik]: https://geofabrik.de
