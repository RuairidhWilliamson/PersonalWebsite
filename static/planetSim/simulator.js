function drawArrow(start, end, colour = constants.COLOUR_DEFUALT, line_width = 1, arrow_tail = new Vector2(3,4)){
	if (!start.equals(end)){
		ctx.beginPath();
		ctx.lineWidth = line_width;
		ctx.strokeStyle = colour;
		ctx.moveTo(end.x, end.y);
		var starttoend = end.subtract(start);
		var perpendicular = starttoend.y != 0 ? new Vector2(1, -starttoend.x /starttoend.y).normalized() : new Vector2(0, 1);
		var arrowVector1 = end.add(starttoend.normalized().multiply(-arrow_tail.y).add(perpendicular.multiply(arrow_tail.x)));
		ctx.lineTo(arrowVector1.x, arrowVector1.y);
		ctx.moveTo(end.x, end.y);
		var arrowVector2 = end.add(end.subtract(start).normalized().multiply(-arrow_tail.y).add(perpendicular.multiply(-arrow_tail.x)));
		ctx.lineTo(arrowVector2.x, arrowVector2.y);
		ctx.moveTo(start.x, start.y);
		var normalizedSTE = starttoend.normalized().multiply(line_width/4);
		ctx.lineTo(end.x + normalizedSTE.x, end.y + normalizedSTE.y);
		ctx.stroke();
	}
}


var canvas = document.querySelector("canvas#sim");
var bodiesDOM = document.querySelector("#bodies");
var ctx = canvas.getContext("2d");
var sim = new Simulation();


var mousedrag_start = new Vector2();
var initial_camPos = new Vector2();
function mousedown(ev){
	mousedrag_start = new Vector2(ev.offsetX, ev.offsetY);
	initial_camPos = sim.cameraPosition;
}
function mousemove(ev){
	if (ev.buttons == 1){
		sim.cameraPosition = initial_camPos.add(new Vector2(ev.offsetX, ev.offsetY).subtract(mousedrag_start).divideBy(sim.zoomlevel));
	}
}
function mousewheel(ev){
	sim.zoomlevel *= 2**(ev.wheelDelta/480);
}
function resize(){
	canvas.width = window.innerWidth -20 - document.querySelector("#bodies").scrollWidth;
	canvas.height = window.innerHeight-10;
}
function playpause(){
	sim.pause = !sim.pause;
	controls.playpause.innerHTML = sim.pause ? "pause_arrow" : "play_arrow";
}
function speed(ev){
	if (!isNaN(ev.srcElement.value)){
		sim.speed = parseFloat(ev.srcElement.value);
	}else{
		ev.srcElement.value = sim.speed;
	}
}
function change_velocity(ev){
	if (!isNaN(ev.srcElement.value)){
		VELOCITY_ARROW = parseFloat(ev.srcElement.value);
	}else{
		ev.srcElement.value = sim.speed;
	}
}
function change_acceleration(ev){
	if (!isNaN(ev.srcElement.value)){
		ACCELERATION_ARROW = parseFloat(ev.srcElement.value);
	}else{
		ev.srcElement.value = sim.speed;
	}
}

var scenariosDOM = document.querySelector("div#scenarios");
for (var i in scenarios){
	scenariosDOM.innerHTML += "<div onclick='scenarios." + i + ".load()' id='" + i + "' class='scenario'>" + scenarios[i].name + "</div>";
}

var last_id = "";
function unique_id (){
	if (performance.now() > last_id){
		last_id = performance.now();
		return performance.now();
	}else{
		last_id += 1;
		return last_id;
	}
}
function string_id(){
	var n = unique_id().toString();
	abc = "abcdefghijklmnopqrstuvwxyz";
	var out = "";
	for (var i in n){
		if (abc[n[i]] != undefined){
			out += abc[n[i]];
		}else{
			out += "0"
		}
	}
	return out;
}
resize();
var controls = {
	"playpause" : document.querySelector("#playpause"),
	"speed" : document.querySelector("#speed"),
	"velocity" : document.querySelector("#velocity-arrow input"),
	"acceleration" : document.querySelector("#acceleration-arrow input"),
}
canvas.addEventListener("mousedown", mousedown);
canvas.addEventListener("mousemove", mousemove);
canvas.addEventListener("mousewheel", mousewheel);
window.addEventListener("resize", resize);
controls.playpause.addEventListener("click", playpause);
controls.speed.addEventListener("change", speed);
controls.velocity.addEventListener("change", change_velocity);
controls.acceleration.addEventListener("change", change_acceleration);
document.querySelector("#addbody").addEventListener("click", function(){
	sim.AddBody(new Body("Custom Object", undefined, 20, undefined, COLOUR_GREY));
});