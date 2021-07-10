<script>
    import { fade } from 'svelte/transition';

    export let segment;
</script>

{#key segment}
    <div in:fade={{duration: 100, delay: 100}} out:fade={{duration: 100}}>
        <slot></slot>
    </div>
{/key}