<script>
	export let segment;

	function toggleDarkMode() {
		window.document.body.classList.toggle('dark');
		window.localStorage.setItem('dark', window.document.body.classList.contains('dark'));
	}

    function onKeyPress() {}
</script>

<style>
	nav {
		border-bottom: 1px solid rgba(255,62,0,0.1);
		font-weight: 300;
		padding: 0 1em;
	}

	:global(body.dark) nav {
		background-color: #1e1e1e;
	}

	ul {
		margin: 0;
		padding: 0;
	}

	/* clearfix */
	ul::after {
		content: '';
		display: block;
		clear: both;
	}

	li {
		display: block;
		float: left;
		transition: background-color 0.1s;
	}

	li:hover {
		background-color: #eee;
	}

	:global(body.dark) li:hover {
		background-color: #333;
	}

	span {
		float: right;
		padding: 1em 0.5em;
		cursor: pointer;
	}

	span:hover {
		color: rgb(255,62,0);
	}

	[aria-current] {
		position: relative;
		display: inline-block;
	}

	[aria-current]::after {
		position: absolute;
		content: '';
		width: calc(100% - 1em);
		height: 2px;
		background-color: rgb(255,62,0);
		display: block;
		bottom: -1px;
	}

	a {
		text-decoration: none;
		padding: 1em 0.5em;
		display: block;
	}
</style>

<nav>
	<ul>
		<li><a aria-current="{segment === undefined ? 'page' : undefined}" href=".">home</a></li>

		<li><a aria-current="{segment === 'projects' ? 'page' : undefined}" href="projects">projects</a></li>
		<li><a aria-current="{segment === 'contact' ? 'page': undefined}" href="contact">contact</a></li>
		<span on:click={toggleDarkMode} on:keypress={onKeyPress}><i class="fas fa-moon"></i></span>
	</ul>
</nav>
