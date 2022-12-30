<script>
	export let status;
	export let error;

	const dev = process.env.NODE_ENV === 'development';
</script>

<style>
	h1, p {
		margin: 0 auto;
	}

	h1 {
		font-size: 2.8em;
		font-weight: 700;
		margin: 0 0 0.5em 0;
	}

	p {
		margin: 1em auto;
	}

	@media (min-width: 480px) {
		h1 {
			font-size: 4em;
		}
	}
</style>

<svelte:head>
	<title>{status}</title>
    <meta name="description" content="The page you are looking for could not be found.">
</svelte:head>

<h1>{status}</h1>

<p>{error.message}</p>
{#if status === 404}
	<p>It looks like the page you were looking for does not exist. <a href="/">Home</a></p>
{/if}

{#if dev && error.stack}
	<pre>{error.stack}</pre>
{/if}
