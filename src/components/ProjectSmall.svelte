<script>
    export let project;

    if (project == undefined) {
        throw 'Undefined project';
    }
</script>

<style>
    a {
        display: block;
        text-decoration: none;
        padding: 1em;
        flex-grow: 1;
		transition: background-color 0.1s;
    }

    a:hover {
        background-color: #eee;
    }

	:global(body.dark) a:hover {
		background-color: #444;
	}

    div.img-container {
        height: 200px;
        margin: 0 0 1em 0;
        text-align: center;
        display: flex;
        flex-direction: column;
        justify-content: end;
    }

    img {
        max-width: 100%;
        max-height: 100%;
        margin: auto;
    }
</style>

<a href="projects/{project.slug}" rel="prefetch">
    <div class="img-container">
        <img src="{project.image}" alt={project.image_alt}/>
    </div>
    <h5>{project.tags.join(', ')}</h5>
    <div>
        <h2>{project.title}</h2>
        <h3>{project.date}</h3>
    </div>
</a>