<script context="module">
	export function preload() {
		return this.fetch(`projects.json`).then(r => r.json()).then(projects => {
			return { projects };
		});
	}
</script>

<script>
	import ProjectSmall from "../components/ProjectSmall.svelte";
	export let projects;

	const featured = [
		'pseudo-haptic-drift-correction',
		'l-systems',
		'trolley-folly',
	];
</script>

<style>
	h1 {
		text-align: center;
		margin: 0 auto;
		font-size: 1em;
		text-transform: uppercase;
		font-weight: 700;
		margin: 0 0 0.5em 0;
	}

	@media (min-width: 480px) {
		h1 {
			font-size: 2em;
		}
	}

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
	<title>Ruairidh Williamson</title>
</svelte:head>

<h1>Ruairidh Williamson</h1>

<h2>Featured Projects</h2>


<div class="projects-container">
	{#each featured as project_slug}
		<ProjectSmall project={projects.find(p => p.slug === project_slug)}/>
	{/each}
</div>