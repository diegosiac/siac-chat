<script lang="ts">
	import SunMoon from 'lucide-svelte/icons/sun-moon';
	import Settings from 'lucide-svelte/icons/settings';

	import * as DropdownMenu from '@/ui/dropdown-menu/index.js';

	import * as Tooltip from '@/ui/tooltip/index.js';
	import { Button } from '@/ui/button';
	import { resetMode, setMode } from 'mode-watcher';

	type Mode = 'system' | 'dark' | 'light';

	const changeMode = (mode: string) => {
		if (mode === 'system') {
			resetMode();
			return;
		}

		setMode(mode as Mode);
	};
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder={dropdownBuilder}>
		<Tooltip.Root openDelay={100}>
			<Tooltip.Trigger asChild let:builder={tooltipBuilder}>
				<Button builders={[dropdownBuilder, tooltipBuilder]} variant="ghost" class="flex flex-col" size="icon">
					<Settings class="h-5 w-5" />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content side="right">
				<p>Configuración</p>
			</Tooltip.Content>
		</Tooltip.Root>
	</DropdownMenu.Trigger>

	<DropdownMenu.Content class="w-56" align="center" side="right">
		<DropdownMenu.Label>Configuración</DropdownMenu.Label>
		<DropdownMenu.Separator />

		<DropdownMenu.Sub>
			<DropdownMenu.SubTrigger>
				<SunMoon class="mr-2 h-4 w-4" />
				<span>Tema</span>
			</DropdownMenu.SubTrigger>
			<DropdownMenu.SubContent>
				<DropdownMenu.RadioGroup
					onValueChange={(value) => {
						if (!value) return;

						changeMode(value);
					}}
				>
					<DropdownMenu.RadioItem value="system">Sistema</DropdownMenu.RadioItem>
					<DropdownMenu.RadioItem value="dark">Oscuro</DropdownMenu.RadioItem>
					<DropdownMenu.RadioItem value="light">Claro</DropdownMenu.RadioItem>
				</DropdownMenu.RadioGroup>
			</DropdownMenu.SubContent>
		</DropdownMenu.Sub>
	</DropdownMenu.Content>
</DropdownMenu.Root>
