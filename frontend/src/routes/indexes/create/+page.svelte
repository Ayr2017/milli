<script>
    import {Section} from "flowbite-svelte-blocks";
    import {Button, Card,  Label, Input} from "flowbite-svelte";

    let index_name = '';

    function handleSubmit() {
        if (!index_name.trim()) return;

        fetch('/api/indexes', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ index_name })
        }).then(response => response.json())
            .then(data => {
                console.log('Index created successfully');
                console.log(data);
                index_name = '';
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
                <form on:submit|preventDefault={handleSubmit}>
                    <div class="mb-6">
                        <Label for="default-input" class="mb-2 block">Name</Label>
                        <Input
                                id="default-input"
                                placeholder="Enter index name"
                                name="index_name"
                                bind:value={index_name}
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