<script lang="ts">
    import { page } from "$app/state";
    import { SidebarBottomNav, SidebarBottomNavItem } from "flowbite-svelte-blocks";
    import { Button, Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownItem, SidebarDropdownWrapper, Dropdown, DropdownItem } from "flowbite-svelte";
    import { ArrowRightAltSolid } from "flowbite-svelte-icons";

    let spanClass = "flex-1 ml-3 whitespace-nowrap";
    let activeUrl = $derived(page.url.pathname);
</script>

<Sidebar position="relative" class="h-screen">
    <SidebarWrapper class="">
        <SidebarGroup>
            <SidebarItem label="Overview" class="cursor-pointer">
                {#snippet icon()}
                <ArrowRightAltSolid size="md"></ArrowRightAltSolid>
                    {/snippet}
            </SidebarItem>
            <SidebarItem label="Indexes" class="cursor-pointer">
                {#snippet icon()}
                <ArrowRightAltSolid size="md"></ArrowRightAltSolid>
                    {/snippet}
            </SidebarItem>
            <SidebarItem label="Data sources" class="cursor-pointer">
                {#snippet icon()}
                <ArrowRightAltSolid size="md"></ArrowRightAltSolid>
                    {/snippet}
            </SidebarItem>
            <SidebarItem label="Documents" class="cursor-pointer">
                {#snippet icon()}
                <ArrowRightAltSolid size="md"></ArrowRightAltSolid>
                    {/snippet}
            </SidebarItem>
        </SidebarGroup>
    </SidebarWrapper>
</Sidebar>