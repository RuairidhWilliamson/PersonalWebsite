
class Vector2{
	constructor (x = 0, y = 0){
		this.x = x;
		this.y = y;
	}
	sqrMagnitude (){
		return this.x * this.x + this.y * this.y;
	}
	magnitude(){
		return Math.sqrt(this.sqrMagnitude(), 0.5);
	}
	normalized(){
		if (this.sqrMagnitude() == 0){
			return new Vector2();
		}
		else{
			return this.divideBy(this.magnitude());
		}
	}
	toString(){
		return "(" + this.x + "," + this.y + ")";
	}
	add (other){
		if (other.NANcheck()){
			throw "NAN Vector";
		}
		return new Vector2(this.x + other.x, this.y + other.y);
	}
	subtract (other){
		if (other.NANcheck()){
			throw "NAN Vector";
		}
		return new Vector2(this.x - other.x, this.y - other.y);
	}
	multiply (scalar){
		return new Vector2(this.x * scalar, this.y * scalar);
	}
	divide (scalar){
		return new Vector2(scalar / this.x, scalar / this.y);
	}
	divideBy (scalar){
		return new Vector2(this.x / scalar, this.y / scalar);
	}
	NANcheck(){
		return (isNaN(this.x) || isNaN(this.y))
	}
	equals(other){
		return this.x == other.x && this.y == other.y;
	}
}