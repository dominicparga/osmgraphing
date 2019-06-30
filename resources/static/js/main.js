//Hardcoded default zoom value
var globalZoom = 10;

//Flag to determine if a new source or dest. marker has to be created
//Only one source and one dest. at the time permitted
var markerTurn = true;

//Object representing a source node
var sourceMarker = {
    id: undefined,
    lat: undefined,
    lng: undefined,
    marker: undefined,
};

//Object representing a destination node
var destMarker= {
    id: undefined,
    lat: undefined,
    lng: undefined,
    marker: undefined,
};


//-----------------------------------------map generation-----------------------------------------------
let map = L.map( 'map', {
    center: [20.0, 5.0],
    minZoom: 2,
    zoom: 2
}).setView(new L.LatLng(48.77490788045187, 9.17959213256836), 8);

L.tileLayer('https://{s}.tile.openstreetmap.de/tiles/osmde/{z}/{x}/{y}.png', {
	maxZoom: 18,
	attribution: '&copy; <a href="http://www.openstreetmap.org/copyright">OpenStreetMap</a>'
}).addTo( map );
//-----------------------------------------map generation-----------------------------------------------

//Init. Map: Create source node visualization at Stuttgart
createSource();

//-----------------------------------------marker generation-----------------------------------------------
/**
 * Handles node identification and marker generation on click
 */
map.on("click", function(e){
    /* $.get("/routing", {"a": 10, "b": 10}, function(data) {
        console.log("sent")
    }) */
    $.ajax({
        url: "/routing",
        //contentType: "application/json; charset=utf-8",
        data: {"a": 10, "b": 42},
        success: function() {
            console.log("sent")
        },
        error: function() {
            console.log("didnt work");
        }
    });
    let coord = map.mouseEventToLatLng(e.originalEvent);
    //let data = getClosestNode(coord.lat, coord.lng);
    //dummy markers
    setInput(coord.lat, coord.lng, 0);
    //dummy markers
});

/**
 * Handles node identification and marker generation on button interaction
 * 
 * @param {String} dir - direction determining if source or destination
 */
function inputPropagation(dir) {
    if(dir === "src") {
        let coord = [$("#lat_src_input").val(), $("#lng_src_input").val()];
        markerTurn = false;
    }
    else {
        let coord = [$("#lat_dest_input").val(), $("#lng_dest_input").val()];
        markerTurn = true;
    }
    let data = getClosestNode(coord[0], coord[1]);
    setInput(data[0], data[1], data[2]);
}

/**
 * Given the the coordinates and the id of a node,
 * the function determines if source or destination has changed
 * using the marker flag. The input values are set and the 
 * corresponding marker is created accordingly.
 * 
 * @param {number} lat - lattitude of the node as float 
 * @param {number} lng - longitute of the node as float
 * @param {number} id  - id of the node as integer
 */
function setInput(lat, lng, id) {
    if(markerTurn == false){
        $("#lat_src_input").val(lat);
        $("#lng_src_input").val(lng);
        createSource(id);
        markerTurn = true;
    } 
    else {
        $("#lat_dest_input").val(lat);
        $("#lng_dest_input").val(lng);
        createDest(id);
        markerTurn = false;
    }
}

/**
 * Creates a new marker for the source node
 * 
 * @param {number} id 
 */
function createSource(id){
    if(sourceMarker.marker){
        map.removeLayer(sourceMarker.marker)
    }
    let coord = [$("#lat_src_input").val(), $("#lng_src_input").val()]
    if(!coord[0] || coord[0] < 0 || !coord[1] || coord[1] < 0){
        alert("Please Enter a valid Destination (Lat,Lng)")
        return
    }
    sourceMarker.marker = L.marker([coord[0], coord[1]]).addTo(map);
    sourceMarker.lat = coord[0];
    sourceMarker.lng = coord[1];
    sourceMarker.id = id;
    sourceMarker.marker.bindPopup("Source: " + coord[0]  + ", " + coord[1]).openPopup();
    map.flyTo(new L.LatLng(coord[0], coord[1], globalZoom));
}

/**
 * Creates a new marker for the destination node
 * 
 * @param {number} id 
 */
function createDest(id){
    if(destMarker.marker){
        map.removeLayer(destMarker.marker)
    }
    let coord = [$("#lat_dest_input").val(), $("#lng_dest_input").val()];
    if(!coord[0] || coord[0] < 0 || !coord[1] || coord[1] < 0){
        alert("Please Enter a valid Destination (Lat,Lng)")
        return
    }
    destMarker.marker = L.marker([coord[0], coord[1]]).addTo(map);
    destMarker.lat = coord[0];
    destMarker.lng = coord[1];
    destMarker.id = id;
    destMarker.marker.bindPopup("Dest.: " + coord[0]  + ", : " + coord[1]).openPopup();
    map.flyTo(new L.LatLng(coord[0], coord[1], globalZoom));
}

/**
 * Given the user specified coordinates of a node, 
 * this function sends a request to the server
 * to determine the closest actual node and its coordinates.
 * 
 * @param {number} lat - lattitude of the node as float
 * @param {number} lng - longitude of the node as float
 * @returns {array} 
 */
function getClosestNode(lat, lng){
    $.get("closest_neighbour", {lat: lat, lng: lng}, function(data) {
        if(!data) {
            alert("No neighbouring node");
        }
        else {
            return data;
        }
    })
}
//-----------------------------------------marker generation-----------------------------------------------


//-----------------------------------------route visualization-----------------------------------------------

function calcRoute(){
    clearRoute();
    if(sourceMarker.length && destMarker.length) {
    }

}
//-----------------------------------------route visualization-----------------------------------------------
