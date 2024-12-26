<script lang="ts">
	import { Button } from '@/ui/button';
	import * as Dialog from '@/ui/dialog';
	import { Input } from '@/ui/input';
	import { Label } from '@/ui/label';
</script>

<Dialog.Content class="sm:max-w-[425px]">
	<Dialog.Header>
		<Dialog.Title>Editar perfil</Dialog.Title>
		<Dialog.Description>
			Haz cambios en tu perfil aquí. Haz click en guardar cuando termines.
		</Dialog.Description>
	</Dialog.Header>
	<div class="grid gap-4 pb-3">
		<div class="space-y-2">
			<Label for="name" class="text-right">Nombre Completo</Label>
			<Input id="name" placeholder="Alan Turing" class="col-span-3" />
		</div>
		<div class="space-y-2">
			<Label for="username" class="text-right">Nombre de usuario</Label>
			<Input id="username" placeholder="@alan.turing" class="col-span-3" />
		</div>
		<div class="space-y-2">
			<Label for="username" class="text-right">Correo electrónico</Label>
			<Input id="username" disabled value="alan@turing.com" class="col-span-3" />
		</div>
	</div>
	<Dialog.Footer>
		<Button type="submit">Guardar cambios</Button>
	</Dialog.Footer>
</Dialog.Content>
