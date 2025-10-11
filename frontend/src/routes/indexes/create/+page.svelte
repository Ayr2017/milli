<script>
    import {Section} from "flowbite-svelte-blocks";
    import {Button, Card,  Label, Input, Modal, P } from "flowbite-svelte";

    let open = $state(false);
    const HTTP_STATUS_OK = 200;

    let name = $state('');

    function handleSubmit() {
        if (!name.trim()) return;

        fetch('/api/indexes', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name })
        }).then(response => {
            return response.json()
        })
            .then(data => {
                if(data.code === HTTP_STATUS_OK) {
                    open = true;
                    name = '';
                } else {
                    console.info(data);
                }
            })
            .catch(error => {
                console.error('Error:', error);
            });
    }
</script>

<div class="mx-auto container my-2">
    <Section>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <Card size="lg" class="p-4 text-left sm:p-8 md:p-10">
                <form onsubmit={handleSubmit}>
                    <div class="mb-6">
                        <Label for="default-input" class="mb-2 block">Name</Label>
                        <Input
                                id="default-input"
                                placeholder="Enter index name"
                                name="name"
                                bind:value={name}
                                required
                        />
                    </div>
                    <div class="flex gap-2">
                        <Button type="button" color="alternative" class="cursor-pointer" href="/indexes">
                            Cancel
                        </Button>
                        <Button type="submit" color="green" class="cursor-pointer">
                            Send
                        </Button>
                    </div>
                </form>
            </Card>
        </div>
    </Section>
</div>

<Modal form bind:open title="Terms of Service">
    <P>Index created successfully!</P>
    {#snippet footer()}
        <Button type="submit" value="accept">I accept</Button>
        <Button type="submit" value="decline" color="alternative">Decline</Button>
    {/snippet}
</Modal>