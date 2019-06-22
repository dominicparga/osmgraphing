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

/* map.on("click", function(e){
    let coord = map.mouseEventToLatLng(e.originalEvent);
    let marker = L.marker([coord.lat, coord.lng]).addTo(map);
    marker.bindPopup("Lat: " + coord.lat  + "\nLng: " + coord.lng).openPopup();;
    marker.on("click", function(){
        marker.openPopup();
    });
}); */

var globalZoom = 10;
var markerTurn = true;
var sourceMarker;
var destMarker;
createSource();

map.on("click", function(e){
    let coord = map.mouseEventToLatLng(e.originalEvent);
    if(markerTurn == false){
        $("#lat_src_input").val(coord.lat);
        $("#lng_src_input").val(coord.lng);
        createSource();
        markerTurn = true;
    } 
    else {
        $("#lat_dest_input").val(coord.lat);
        $("#lng_dest_input").val(coord.lng);
        createDest();
        markerTurn = false;
    }

});

function createSource(){
    if(sourceMarker){
        map.removeLayer(sourceMarker)
    }
    let coord = [$("#lat_src_input").val(), $("#lng_src_input").val()]
    if(!coord[0] || coord[0] < 0 || !coord[1] || coord[1] < 0){
        alert("Please Enter a valid Destination (Lat,Lng)")
        return
    }
    sourceMarker = L.marker([coord[0], coord[1]]).addTo(map);
    map.flyTo(new L.LatLng(coord[0], coord[1], globalZoom));
    sourceMarker.bindPopup("Lat: " + coord[0]  + "\nLng: " + coord[1]).openPopup();;
}

function createDest(){
    if(destMarker){
        map.removeLayer(destMarker)
    }
    let coord = [$("#lat_dest_input").val(), $("#lng_dest_input").val()];
    if(!coord[0] || coord[0] < 0 || !coord[1] || coord[1] < 0){
        alert("Please Enter a valid Destination (Lat,Lng)")
        return
    }
    destMarker = L.marker([coord[0], coord[1]]).addTo(map);
    map.flyTo(new L.LatLng(coord[0], coord[1], globalZoom));
    destMarker.bindPopup("Lat: " + coord[0]  + "\nLng: " + coord[1]).openPopup();
}
