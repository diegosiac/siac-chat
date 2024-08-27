<script>
	import { SideBar } from '@/components';
	import '../app.css';
	import { Toaster } from '@/ui/sonner';
	import { ModeWatcher } from 'mode-watcher';
</script>

<ModeWatcher />
<Toaster />

<div class="flex gap-3 p-3">
	<SideBar />

	<main class="w-full flex gap-3">
		<slot></slot>
	</main>
</div>
