<script lang="ts">
	import LogOut from 'lucide-svelte/icons/log-out';
	import User from 'lucide-svelte/icons/user';
	import SunMoon from 'lucide-svelte/icons/sun-moon';
	import { signOut, signIn } from '@auth/sveltekit/client';
	import { page } from '$app/stores';
	import * as DropdownMenu from '@/ui/dropdown-menu';
	import { Root as Dialog, Trigger as DialogTrigger } from '@/ui/dialog';
	import * as Tooltip from '@/ui/tooltip/index.js';
	import { Button } from '@/ui/button';

	import { toggleMode } from 'mode-watcher';
	import { ContentDialogAuth } from '@/features/auth/components';
	import { ContentDialogProfile } from '@/features/user/components';

	const toggleTheme = () => {
		toggleMode();
	};
</script>

<DropdownMenu.Root>
	<Dialog>
		<DropdownMenu.Trigger asChild let:builder={dropdownBuilder}>
			<Tooltip.Root openDelay={100}>
				<Tooltip.Trigger asChild let:builder={tooltipBuilder}>
					<Button
						builders={[dropdownBuilder, tooltipBuilder]}
						variant="ghost"
						class="flex flex-col"
						size="icon"
					>
						<User class="h-5 w-5" />
					</Button>
				</Tooltip.Trigger>
				<Tooltip.Content side="right">
					<p>Perfil</p>
				</Tooltip.Content>
			</Tooltip.Root>
		</DropdownMenu.Trigger>

		<DropdownMenu.Content class="w-56" align="center" side="right">
			<DropdownMenu.Label>Mi Perfil</DropdownMenu.Label>
			<DropdownMenu.Separator />
			<DropdownMenu.Item on:click={toggleTheme}>
				<SunMoon class="mr-2 h-4 w-4" />
				<span>Cambiar Tema</span>
			</DropdownMenu.Item>
			{#if $page.data.session}
				<DialogTrigger class="w-full">
					<DropdownMenu.Item>
						<User class="mr-2 h-4 w-4" />
						<span>Perfil</span>
					</DropdownMenu.Item>
				</DialogTrigger>
				<DropdownMenu.Item on:click={() => signOut()}>
					<LogOut class="mr-2 h-4 w-4" />
					<span>Cerrar sesión</span>
				</DropdownMenu.Item>
			{:else}
				<DropdownMenu.Item on:click={() => signIn('credentials')}>
					<User class="mr-2 h-4 w-4" />
					<span>Iniciar falso</span>
				</DropdownMenu.Item>
				<DialogTrigger class="w-full">
					<DropdownMenu.Item>
						<User class="mr-2 h-4 w-4" />
						<span>Iniciar sesión</span>
					</DropdownMenu.Item>
				</DialogTrigger>
			{/if}
		</DropdownMenu.Content>
		{#if $page.data.session}
			<ContentDialogProfile />
		{:else}
			<ContentDialogAuth />
		{/if}
	</Dialog>
</DropdownMenu.Root>
