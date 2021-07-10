<script>
	import TransitionWrapper from '../../components/TransitionWrapper.svelte';

    export let segment;
</script>

<TransitionWrapper {segment}>
    <slot></slot>
</TransitionWrapper>