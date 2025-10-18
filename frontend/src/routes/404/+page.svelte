<script>
    import { Section, Page404 } from "flowbite-svelte-blocks";
    import { Button } from "flowbite-svelte";
    import { Laptop404NotFoundError, Laptop404NotFoundErrorDark } from "flowbite-svelte-illustrations";
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    onMount(() => {
        // Перенаправляет на главную страницу через 3 секунды
        setTimeout(() => {
            goto('/');
        }, 3000);
    });
</script>


<Section name="page404">
    <div class="">
        <Laptop404NotFoundError />
    </div>
<!--    <div class="hidden dark:block">-->
<!--        <Laptop404NotFoundErrorDark />-->
<!--    </div>-->

<!--    <Page404>-->
<!--        {#snippet h1()}404{/snippet}-->
<!--        {#snippet paragraph()}-->
<!--            <p class="mb-4 text-3xl font-bold tracking-tight text-gray-900 md:text-4xl dark:text-white">Something's missing.</p>-->
<!--            <p class="mb-4 text-lg font-light text-gray-500 dark:text-gray-400">Sorry, we can't find that page. You'll find lots to explore on the home page.</p>-->
<!--            <Button href="/" size="lg" color="red">Back to Homepage</Button>-->
<!--        {/snippet}-->
<!--    </Page404>-->
</Section>