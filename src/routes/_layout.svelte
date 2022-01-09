<script>
    import Nav from '../components/Nav.svelte';
    import TransitionWrapper from '../components/TransitionWrapper.svelte';

    if (typeof window !== 'undefined') {
        if (window.localStorage.getItem('dark') === 'true') {
		    window.document.body.classList.add('dark');
        }
        window.document.body.classList.add('loaded');
    }

    export let segment;
</script>

<style>
    main {
        position: relative;
        max-width: 56em;
        padding: 2em;
        margin: 0 auto;
        box-sizing: border-box;
    }
</style>

<Nav {segment}/>
<main>
    <TransitionWrapper {segment}>
        <slot></slot>
    </TransitionWrapper>
</main>