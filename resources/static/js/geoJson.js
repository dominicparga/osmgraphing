var geoLayer = L.geoJSON().addTo(map);
var route;
var routeLayer;
var routeStyle = {
    "color" : "#ff7800",
    "weight": 5,
    "opacity": 0.65
};

function clearRoute(){
    if(routeLayer){
        map.removeLayer(routeLayer);
    }
}

function createRoute(distance, route){
    route = [
        {
            "type" : "LineString",
            "coordinates" : route
        }
    ];

    var routeFeature = {
        "type" : "feature",
        "properties" : {
            "popupContent": "distance: " + distance
        }
    };

    routeLayer = L.geoJSON(route, {style : routeStyle});
    routeLayer.addTo(map);
    routeLayer.bindPopup(routeFeature.properties.popupContent).openPopup();
}