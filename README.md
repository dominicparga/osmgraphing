# osmgraphing

Goal of this student project is parsing [openstreetmap](openstreetmap.org) data to calculate traffic routes on it.

## Goal

Long story short, the pipeline will look like this:

1. Download `raw osm data`
2. Read in `raw osm data` (probably partially)
3. Filter tags
4. Merge and process `osm-nodes` and `osm-edges`
5. Create `streetgraph`

### Download `raw osm data`

Downloaded osm data is provided in xml, where nodes are related to location in latitude and longitude.

Problems will be the size limit of downloading from [openstreetmap](openstreet.org), but there are other osm data providers like [geofabrik](download.geofabrik.de) for instance.

### Read `raw osm data`

Since osm data is stored in huge xml files (up to GB), reading in those files to parse them could be tricky.

### Filter tags

For routing, traffic crossroads are needed and many tags of the provided data is not needed.
So first step will be providing osm data and filtering unnecessary tags in a parsing step.

### Merge and process `osm nodes` and `osm edges`

To do routing on the remaining data, the parsing result has to be stored efficiently in a graph structure.
Since nodes are location related, not traffic-logic related, osm-nodes or osm-edges have to be merged into crossroads and edges, so routing algorithms don't have to execute useless steps.

### Create streetgraph

An offset-graph will fulfill the needs of quick node accesses.
A metric for calculating lengths is needes as well.
`Haversine distance` is the distance (e.g. in meters) between two points on a sphere (given in latitude and longitude).

## TODO

- Check `leaflet` - osm data provider?
- Write info about filtered tags (count them explicitly)
- Check order of latitude and longitude since changing it during development is quite annoying ._.
