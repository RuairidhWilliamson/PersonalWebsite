<script context="module">
	export function preload() {
		return this.fetch(`projects.json`).then(r => r.json()).then(projects => {
			return { projects };
		});
	}
</script>

<script>
	import ProjectSmall from "../../components/ProjectSmall.svelte";

	export let projects;
</script>

<style>
	.projects-container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		grid-auto-flow: dense;
		gap: 1rem;
		margin: 0 0 1em 0;
		line-height: 1.5;
	}
</style>

<svelte:head>
	<title>Projects</title>
    <meta name="description" content="List of projects by Ruairidh Williamson.">
</svelte:head>

<h1>Projects</h1>

<div class="projects-container">
	{#each projects as project}
		<ProjectSmall project={project}/>
	{/each}
</div>
