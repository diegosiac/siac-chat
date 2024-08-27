<script lang="ts">
	import LogOut from 'lucide-svelte/icons/log-out';
	import User from 'lucide-svelte/icons/user';
	import SunMoon from 'lucide-svelte/icons/sun-moon';

	import * as DropdownMenu from '@/ui/dropdown-menu/index.js';

	import * as Tooltip from '@/ui/tooltip/index.js';
	import { Button } from '@/ui/button';


	import { toggleMode } from 'mode-watcher';

  const toggleTheme = () => {
    toggleMode();
  }

</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder={dropdownBuilder}>
		<Tooltip.Root openDelay={100}>
			<Tooltip.Trigger asChild let:builder={tooltipBuilder}>
				<Button builders={[dropdownBuilder, tooltipBuilder]} variant="ghost" class="flex flex-col" size="icon">
					<User class="h-5 w-5" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">
				<p>Perfil</p>
			</Tooltip.Content>
		</Tooltip.Root>
	</DropdownMenu.Trigger>

	<DropdownMenu.Content class="w-56" align="center" side="right">
		<DropdownMenu.Label>Mi cuenta</DropdownMenu.Label>
		<DropdownMenu.Separator />
		<DropdownMenu.Group>
      <DropdownMenu.Item on:click={toggleTheme}>
				<SunMoon class="mr-2 h-4 w-4" />
				<span>Cambiar Tema</span>
			</DropdownMenu.Item>
			<DropdownMenu.Item>
				<User class="mr-2 h-4 w-4" />
				<span>Perfil</span>
			</DropdownMenu.Item>
		</DropdownMenu.Group>
		<DropdownMenu.Item>
			<LogOut class="mr-2 h-4 w-4" />
			<span>Cerrar sesión</span>
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
