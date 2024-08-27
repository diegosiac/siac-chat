<script lang="ts">
	import { buttonVariants } from '@/ui/button';
	import { cn } from '@/utils';

	import * as Tooltip from '@/ui/tooltip/index.js';

  type $$Props = {
    href: string;
    class?: string;
    title: string;
  }

  export let title: $$Props['title'] = '';

</script>

<Tooltip.Root openDelay={100}>
	<Tooltip.Trigger asChild let:builder>
		<a
			href="/hola"
			class={cn(buttonVariants({ variant: 'ghost', size: 'icon' }), $$restProps.class)}
			use:builder.action
			{...builder}
			{...$$restProps}
		>
			<slot />
      <span class="sr-only">{title}</span>
		</a>
	</Tooltip.Trigger>

	<Tooltip.Content side="right">
		<p>{title}</p>
	</Tooltip.Content>
</Tooltip.Root>
