<script lang="ts">
	import * as Button from '@/components/button';

	type $$Props = Button.Props;
	type $$Events = Button.Events;
</script>

<Button.Root type="submit" on:click on:keydown {...$$restProps}>
	<slot />
</Button.Root>
