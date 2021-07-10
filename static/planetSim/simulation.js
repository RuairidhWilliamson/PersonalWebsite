
class Simulation {
	constructor(){
		this.bodies = [];
		this.time = 0;
		this.speed = 1;
		this.pause = 0;
		this.updatesPerSecond = 30;
		this.lastRender = 0;
		this.fps_accuracy = 0;
		this.cameraLock = undefined;
		this.cameraPosition = new Vector2();
		this.finalCameraPosition = new Vector2();
		this.zoomlevel = 1;
		this.constants = {
			"GRAVITATIONAL_CONSTANT" : GRAVITATIONAL_CONSTANT
		}
		this.global_forces = [];

	}
	Update(){
		var deltaTime = this.speed/this.updatesPerSecond;
		if (this.pause == 1){
			//Begin update
			for (var i in this.bodies){
				this.bodies[i].UpdateBegin(this.bodies);
			}
			//After update
			for (var i in this.bodies){
				this.bodies[i].UpdateAfter(deltaTime);
			}
		}
		if (this.cameraLock != undefined){
			this.finalCameraPosition = this.cameraPosition.subtract(this.cameraLock.displacement);
		}else{
			this.finalCameraPosition = this.cameraPosition;
		}
		//Render
		ctx.fillStyle = COLOUR_WHITE;
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		for (var i in this.bodies){
			this.bodies[i].Render();
		}
		//Global Forces
		for (var f in sim.global_forces){
			drawArrow(new Vector2(canvas.width-40, 40), new Vector2(canvas.width-40, 40).add(sim.global_forces[f].normalized().multiply(30)), COLOUR_ORANGE)
		}

		//Framerate Display
		var tnow = performance.now();
		var m = 10 ** this.fps_accuracy;
		ctx.fillStyle = COLOUR_BLACK;
		ctx.fillText("Framerate: " + Math.round(1000 * m / (tnow - this.lastRender)) / m, 0, 10);
		ctx.fillText("Bodies: " + this.bodies.length, 0, 22);
		this.lastRender = performance.now();
	}
	Stop(){
		clearInterval(this.interval);
		bodiesDOM.innerHTML = "";
	}
	StartLoop (){
		controls.playpause.innerHTML = this.pause ? "pause_arrow" : "play_arrow";
		clearInterval(this.interval);
		var s = this;
		this.interval = setInterval(function(){s.Update()}, 1000/this.updatesPerSecond);
		controls.speed.value = this.speed;
	}
	WorldToScreen ( position){
		return position.add(this.finalCameraPosition).multiply(this.zoomlevel).add(new Vector2(canvas.width/2, canvas.height/2));
	}
	AddBody(body){
		this.bodies.push(body);
		var properties = {
			"Name" : "name",
			"Displacement" : "displacement",
			"Velocity" : "velocity",
			"Speed" : "Speed",
			"Acceleration" : "acceleration",
			"Radius" : "radius",
			"Mass" : "mass",
			"Colour" : "colour",
		}
		var out = "";
		for (var i in properties){
			out += "<tr id='" + properties[i] + "' class='prop'><td>"+ i +"</td>";
			if (body[properties[i]] instanceof Vector2){
				out += "<td><input id='" + body.id + "' class='propval' type='number' value='" + body[properties[i]].x + "'/><input type='number' id='" + body.id + "' class='propval' value='" + body[properties[i]].y + "'/></td></tr>";
			}else if (typeof body[properties[i]] == "number"){
				out += "<td><input id='" + body.id + "' class='propval' type='number' value='" + body[properties[i]] + "'/></td></tr>";
			}else if (typeof body[properties[i]] == "string"){
				if (properties[i] == "colour"){
					out += "<td><input type='color' id='" + body.id + "' class='propval' value'" + body[properties[i]] + "'/></td></tr>";
				}else{
					out += "<td><input id='" + body.id + "' class='propval' value'" + body[properties[i]] + "'/></td></tr>";
				}
			}else if (typeof body[properties[i]] == "function"){
				out += "<td><input id='" + body.id + "' class='propval' value='" + body[properties[i]] + "'/></td></tr>";
			}else{
				out += "<td></td></tr>";
			}
		}
		bodiesDOM.innerHTML += "<div id='" + body.id + "' class='body'><div onclick='sim.bodies[" + (this.bodies.length-1) + "].Expand()' class='top'><i class='material-icons expand'>expand_more</i><span>" + body.name + "</span></div><i onclick='sim.bodies[" + (this.bodies.length-1) + "].SetFocus()' class='focusBody material-icons'>open_with</i><div class='expandable'><table>" + out + "</table></div></div>";
		setTimeout(function(){
			for (var b in sim.bodies){
				body = sim.bodies[b];
				body.propertiesDOMS = {};
				for (var i in properties){
					if (body[properties[i]] instanceof Vector2){
						body.propertiesDOMS[properties[i]] = document.querySelectorAll("tr#"+properties[i]+" td input#" + body.id);
						for (var j = 0; j < body.propertiesDOMS[properties[i]].length; j += 1){
							body.propertiesDOMS[properties[i]][j].addEventListener("change", function(ev){if (sim.pause){}ev.srcElement.takeVal = 'true';});
						}
					}else{
						body.propertiesDOMS[properties[i]] = document.querySelector("tr#"+properties[i]+" td input#" + body.id);
						body.propertiesDOMS[properties[i]].addEventListener("change", function(ev){if (sim.pause){}ev.srcElement.takeVal = 'true';});
					}
				}
			}
		},100);
	}
	AddBodies(bodies){
		for (var i in bodies){
			this.AddBody(bodies[i]);
		}
	}
}