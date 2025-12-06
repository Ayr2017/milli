<script lang="ts">
    import { Button, Modal, type ModalProps, P } from "flowbite-svelte";

    let openSourceQueryModal = $state(false);
    let {queryText} = $props();

    function onclick() {
        openSourceQueryModal = true;
    }

</script>
<Button size="xs" outline onclick={() => onclick()}>Show query</Button>

<Modal form title="Query" bind:open={openSourceQueryModal} size="xl">
    <P>
        {queryText}
    </P>
    {#snippet footer()}
        <Button type="submit">I accept</Button>
        <Button type="submit" color="alternative">Decline</Button>
    {/snippet}
</Modal>