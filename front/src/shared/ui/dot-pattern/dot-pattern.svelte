<script lang="ts">
	import { cn } from '@/utils';

	type $$Props = {
		width?: any;
		height?: any;
		x?: any;
		y?: any;
		cx?: any;
		cy?: any;
		cr?: any;
		className?: string;
		[key: string]: any;
	};

	export let width: $$Props['width'] = 16;
	export let height: $$Props['height'] = 16;
	export let x: $$Props['x'] = 0;
	export let y: $$Props['y'] = 0;
	export let cx: $$Props['cx'] = 1;
	export let cy: $$Props['cy'] = 1;
	export let cr: $$Props['cr'] = 1;
	export let className: $$Props['className'] = '';

	const id = `pattern-${Math.random().toString(36).substr(2, 9)}`;
</script>

<svg
	aria-hidden="true"
	class={cn('pointer-events-none absolute inset-0 h-full w-full fill-neutral-400/80', className)}
	{...$$restProps}
>
	<defs>
		<pattern
			{id}
			{width}
			{height}
			patternUnits="userSpaceOnUse"
			patternContentUnits="userSpaceOnUse"
			{x}
			{y}
		>
			<circle id="pattern-circle" {cx} {cy} r={cr} />
		</pattern>
	</defs>
	<rect width="100%" height="100%" stroke-width={0} fill={`url(#${id})`} />
</svg>

