<script lang="ts">
    import {Button, Modal, type ModalProps, P} from "flowbite-svelte";
    import hljs from 'highlight.js';


    let openSourceQueryModal = $state(false);
    let {queryText} = $props();
    const highlightedCode = hljs.highlight(
        queryText,
        { language: 'sql' }
    ).value

    function onclick() {
        openSourceQueryModal = true;
    }

</script>
<Button size="xs" outline onclick={() => onclick()}>Show query</Button>

<Modal form title="Query" bind:open={openSourceQueryModal} size="xl">
    <P>
        {@html highlightedCode}
    </P>

    {#snippet footer()}
        <Button type="submit">I accept</Button>
        <Button type="submit" color="alternative">Decline</Button>
    {/snippet}
</Modal>