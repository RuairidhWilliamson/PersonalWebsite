class Force {
	constructor (by, to, force = new Vector2(), type = FORCE_UNDEFINED){
		this.force = force;
		this.type = type;
		this.by = by;
		this.to = to;
	}
}