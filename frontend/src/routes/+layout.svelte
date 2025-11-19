<script>
    import '../app.css';
    import favicon from '$lib/assets/favicon.ico';
    import logo from '$lib/assets/android-chrome-192x192.png';
    import {page} from '$app/state';

    let activeUrl = $derived(page.url.pathname);


    let {children} = $props();
    import {Navbar, NavBrand, NavLi, NavUl, NavHamburger, DarkMode, Tabs, TabItem} from "flowbite-svelte";
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>

<Navbar class="bg-white border-gray-200 dark:bg-gray-900">
    <NavBrand href="/">
        <img src="{logo}" class="me-3 h-6 sm:h-9 rounded-full" alt="Flowbite Logo"/>
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">Milli</span>
    </NavBrand>
    <NavHamburger/>
    <NavUl {activeUrl}>
        <NavLi href="/">Home</NavLi>
        <NavLi href="/about">About</NavLi>
        <NavLi href="/dashboard">Dashboard</NavLi>
        <NavLi href="/indexes">Indexes</NavLi>
        <NavLi href="/search-settings">Search settings</NavLi>
        <NavLi href="/api-keys">Api keys</NavLi>
        <NavLi href="/data-sources">Data sources</NavLi>
        <NavLi href="/about">Analytics</NavLi>
        <DarkMode/>
    </NavUl>
</Navbar>

{@render children?.()}