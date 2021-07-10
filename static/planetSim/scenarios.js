
var scenarios = {
	"empty" : {
		name: "Empty",
		load : function(){
			sim.Stop();
			sim = new Simulation();
			sim.StartLoop();
		}
	},
	"basic" : {
		name: "2 Bodies",
		load : function(){
			sim.Stop();
			sim = new Simulation();

			var b1 = new Body("Body 1", 1, 20, BODY_UNDEFINED, COLOUR_BLUE);

			b1.velocity = new Vector2(100, 0 );
			var b2 = new Body("Body 2", 1, 20, BODY_UNDEFINED, COLOUR_RED);
			b2.displacement = new Vector2(0, 200);
			b2.velocity = new Vector2(-80, 0);

			sim.AddBodies([b1, b2]);
			sim.StartLoop();
		}
	},
	"threebodies" : {
		name: "3 Bodies",
		load : function(){
			sim.Stop();

			sim = new Simulation();
			var b1 = new Body("Body 1", 1, 10, BODY_UNDEFINED, COLOUR_BLUE);
			b1.displacement = new Vector2(-100, 0);
			b1.velocity = new Vector2(0, 100);
			
			var b3 = new Body("Body 2", 1, 10, BODY_UNDEFINED, COLOUR_GREEN);
			b3.displacement = new Vector2(0, 0);
			b3.velocity = new Vector2(0, 0);

			var b2 = new Body("Body 3", 1, 10, BODY_UNDEFINED, COLOUR_BLUE);
			b2.displacement = new Vector2(100, 0);
			b2.velocity = new Vector2(0, -100);

			

			sim.cameraLock = b1;
			sim.AddBodies([b1, b3, b2]);
			sim.StartLoop();
		}
	},
	"threebodieschaos" : {
		name: "3 Bodies (Chaotic)",
		load : function(){
			sim.Stop();

			sim = new Simulation();
			var b1 = new Body("Body 1", 1, 40, BODY_UNDEFINED, COLOUR_RED);
			b1.displacement = new Vector2(0, 0);
			b1.velocity = new Vector2(0, 0);
			
			var b2 = new Body("Body 2", 1, 20, BODY_UNDEFINED, COLOUR_BLUE);
			b2.displacement = new Vector2(200, 10);
			b2.velocity = new Vector2(0, -250);

			var b3 = new Body("Body 3", 1, 10, BODY_UNDEFINED, COLOUR_GREEN);
			b3.displacement = new Vector2(300, 0);
			b3.velocity = new Vector2(0, 240);

			sim.cameraLock = b1;
			sim.AddBodies([b1, b2, b3]);
			sim.StartLoop();
			
		}
	},
	"earthmoon1" : {
		name : "Earth - Moon (Not to Scale)",
		load : function (){
			sim.Stop();
			sim = new Simulation();
			var earth = new Body("Earth", 1, 18, BODY_PLANET, COLOUR_GREEN);
			earth.displacement = new Vector2(0, 0);

			var moon = new Body("Moon", 1, 5, BODY_SATTELITE, COLOUR_GREY);
			moon.displacement = new Vector2(0, 300);
			moon.velocity = new Vector2(100,0);
			
			sim.cameraLock = earth;
			sim.AddBodies([earth, moon]);
			sim.StartLoop();
		}
	},
	"sem" : {
		name : "Sun Earth Moon (Not to Scale)",
		load : function(){
			sim.Stop();
			sim = new Simulation();
			sun = new Body("Sun", 1, 400, BODY_STAR, COLOUR_YELLOW);

			earth = new Body("Earth", 1, 20, BODY_PLANET, COLOUR_GREEN);
			earth.displacement = new Vector2(0, 1600);
			earth.velocity = new Vector2(1000, 0);

			moon = new Body("Moon", 1, 5, BODY_SATTELITE, COLOUR_GREY);
			moon.displacement = new Vector2(0, 1650);
			moon.velocity = new Vector2(1300, 0);

			sim.cameraLock = earth;
			sim.speed = 0.1;
			sim.AddBodies([sun, earth, moon]);
			sim.StartLoop();
		}
	},
	"chaoticsolar" : {
		name : "Solar System",
		load : function(){
			sim.Stop();
			sim = new Simulation();

			sun = new Body("Sun", 1, 100, BODY_STAR, COLOUR_YELLOW);
			var bodies = [sun];
			for (var i = 1; i <= 10; i += 1){
				var t = new Body("Body " + i, 1, 10, BODY_PLANET, COLOUR_GREY);
				var r = i * 400 + 150;
				t.displacement = new Vector2(r, 0);
				t.velocity = new Vector2(0, Math.sqrt((10**8)/r));
				bodies.push(t);
			}
			sim.cameraLock = sun;
			sim.pause = 0;
			sim.AddBodies(bodies);
			sim.StartLoop();
		}
	},
	"solarwind" : {
		name : "Solar Wind",
		load : function(){
			sim.Stop();
			sim = new Simulation();

			earth = new Body("Earth", 1, 100, BODY_PLANET, COLOUR_GREEN);
			earth.displacement = new Vector2();
			earth.velocity = new Vector2();

			moon = new Body("Moon", 1, 20, BODY_SATTELITE, COLOUR_GREY);
			moon.displacement = new Vector2(600, 0);
			moon.velocity = new Vector2(0, -400);

			sim.global_forces.push(new Vector2(0, -2000));
			sim.cameraLock = earth;
			sim.AddBodies([earth, moon]);
			sim.StartLoop();
		}
	},
	"chaoticsys" : {
		name : "Chaotic System",
		load : function (){
			sim.Stop();
			sim = new Simulation();
			star = new Body("Star", 1, 100, BODY_STAR, COLOUR_ORANGE);
			var bs = [star];
			var c = 10;
			for (var i = 1-c; i < c; i+=1){
				if (i != 0){
					var b = new Body("H" + (c + i), 1, 10, BODY_UNDEFINED, COLOUR_BLUE);
					b.displacement = new Vector2(400 * i + 200, 0);
					b.mass = 10;
					b.velocity = new Vector2(0, Math.sqrt(GRAVITATIONAL_CONSTANT * star.mass / (400*Math.abs(i)))* Math.sign(i) * ((Math.abs(i) % 2)*2-1));
					bs.push(b);
				}
			}
			for (var i = 1-c; i < c; i+=1){
				if (i != 0){
					var b = new Body("V" + (c + i), 1, 10, BODY_UNDEFINED, COLOUR_GREEN);
					b.displacement = new Vector2(0, 400 * i);
					b.mass = 10;
					b.velocity = new Vector2(Math.sqrt(GRAVITATIONAL_CONSTANT * star.mass / (400*Math.abs(i)))* Math.sign(i) * ((Math.abs(i) % 2)*2-1), 0);
					bs.push(b);
				}
			}
			sim.AddBodies(bs);
			sim.StartLoop();
		}
	}
}