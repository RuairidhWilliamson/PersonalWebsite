class Body {
	constructor (name = "unnamed", massMultiplier = 1, radius = 1, type = BODY_UNDEFINED, colour = COLOUR_DEFUALT){
		this.name = name;
		this.id = string_id();
		this.mass = massMultiplier * radius ** 2;
		this.radius = radius;
		this.type = type;
		this.colour = colour;
		this.displacement = new Vector2();
		this.velocity = new Vector2();
		this.acceleration = new Vector2();
		this.resultant_force = new Vector2();
		this.trail = [];
		this.trailLength = 5000;
		this.propertyExpanded = false;
		this.propertiesDOMS = {};
		this.destroyed = false;
	}
	Speed(){
		return this.velocity.magnitude();
	}
	
	//Calculate forces
	UpdateBegin(bodies){
		if (!this.destroyed){
			this.forces = [];
			this.resultant_force = new Vector2();
			for (var i in bodies){
				var other = bodies[i];
				if (other != this && !other.destroyed){
					var magBetween = this.displacement.subtract(other.displacement).sqrMagnitude();
					if (this.mass >= other.mass && magBetween < (this.radius + other.radius) ** 2){
						//On collision join bodies
						other.destroyed = true;
						this.radius = new Vector2(this.radius, other.radius).magnitude();
						//Conservation of momentum
						this.velocity = this.velocity.multiply(this.mass).add( other.velocity.multiply(other.mass)).divideBy(this.mass + other.mass);
						this.mass += other.mass;
						document.querySelector("div.body#"+ other.id + " .top span").classList.add("destroyed");
					}
					var gravitional = new Force( other, this, undefined, FORCE_GRAVITATIONAL);
					//F = GMm / r^2
					if (magBetween != 0){	
						gravitional.force = other.displacement.subtract(this.displacement).normalized().multiply( sim.constants.GRAVITATIONAL_CONSTANT * this.mass * other.mass / magBetween);
						if (gravitional.force.NANcheck()){
							console.log("ERROR");
						}
						this.resultant_force = this.resultant_force.add(gravitional.force);
					}
					for (var f in sim.global_forces){
						this.resultant_force = this.resultant_force.add(sim.global_forces[f]);
					}
				}
			}
		}
	}
	UpdateAfter(deltaTime){
		//Update displacement, velocity, acceleration
		//a = F/m
		if (!this.destroyed){
			this.acceleration = this.resultant_force.divideBy(this.mass);
			this.velocity = this.velocity.add(this.acceleration.multiply(deltaTime));
			this.displacement = this.displacement.add(this.velocity.multiply(deltaTime));
			this.trail.push(this.displacement);
			
		}else{
			this.trail.push(new Vector2(100000,100000));
		}
		if (this.trail.length > this.trailLength / sim.bodies.length){
			this.trail.shift();
		}
	}
	Render(){
		//Rendering
		if (!this.destroyed){
			ctx.beginPath();
			ctx.fillStyle = this.colour;
			var d = sim.WorldToScreen(this.displacement);
			ctx.arc(d.x, d.y, this.radius*sim.zoomlevel, 0, 2*Math.PI);
			ctx.fill();
			
			drawArrow(
				sim.WorldToScreen(this.displacement),
				sim.WorldToScreen(this.displacement.add(this.velocity.multiply(VELOCITY_ARROW))), 
				"#C51162", 
				sim.zoomlevel, 
				new Vector2(3,4).multiply(sim.zoomlevel)
			);
			drawArrow(
				sim.WorldToScreen(this.displacement), 
				sim.WorldToScreen(this.displacement.add(this.acceleration.multiply(ACCELERATION_ARROW))), 
				"#4A148C", 
				sim.zoomlevel, 
				new Vector2(3,4).multiply(sim.zoomlevel)
			);
			//Update properties
			var vectorConversion = {
				0 : "x",
				1 : "y",
				2 : "z",
			}
			if (this.propertyExpanded){
				for (var i in this.propertiesDOMS){
					if (this.propertiesDOMS[i] instanceof NodeList){
						for (var j = 0; j < this.propertiesDOMS[i].length; j+= 1){
							if (!sim.pause && this.propertiesDOMS[i][j].takeVal == 'true'){
								if (!isNaN(this.propertiesDOMS[i][j].value)){
									this[i][vectorConversion[j]] = parseFloat(this.propertiesDOMS[i][j].value);
								}
								this.propertiesDOMS[i][j].takeVal = 'false';
							}
							if (this.propertiesDOMS[i][j] != document.activeElement || sim.pause){
								this.propertiesDOMS[i][j].value = Math.round(this[i][vectorConversion[j]], 1);
							}
						}
					}else if ( typeof this[i] == "string"){
						if (!sim.pause && this.propertiesDOMS[i].takeVal == 'true'){
							this[i] = this.propertiesDOMS[i].value;
							if (i == "name"){
								document.querySelector("div#"+this.id + " div.top span").innerHTML = this.name;
							}
							this.propertiesDOMS[i].takeVal = 'false';
						}
						if (this.propertiesDOMS[i] != document.activeElement || sim.pause){
							this.propertiesDOMS[i].value = this[i];
						}
					}else if (typeof this[i] == "function"){
						this.propertiesDOMS[i].value = this[i]();
					}else{
						if (!sim.pause && this.propertiesDOMS[i].takeVal == 'true'){
							if (!isNaN(this.propertiesDOMS[i].value)){
								this[i] = parseFloat(this.propertiesDOMS[i].value);
							}
							this.propertiesDOMS[i].takeVal = 'false';
						}
						if (this.propertiesDOMS[i] != document.activeElement || sim.pause){
							this.propertiesDOMS[i].value = Math.round(this[i],1);
						}
					}
				}
			}
		}
		for (var i in this.trail){
			ctx.fillStyle = COLOUR_BLACK;
			var t = sim.WorldToScreen(this.trail[i]);
			ctx.fillRect(t.x - 0.5 * sim.zoomlevel, t.y - 0.5 * sim.zoomlevel, 0.5 * sim.zoomlevel, 0.5 * sim.zoomlevel);
		}
	}
	Expand(ev){
		this.propertyExpanded = document.querySelector("#" + this.id + " .expandable").classList.toggle("open");
		var expand_symbol = document.querySelector("#" + this.id + " .expand");
		expand_symbol.innerHTML = expand_symbol.innerHTML == "expand_more" ? "expand_less" : "expand_more";
	}
	SetFocus(){
		if (!this.destroyed){
			sim.cameraLock = this;
			sim.cameraPosition = new Vector2();
		}
	}
}