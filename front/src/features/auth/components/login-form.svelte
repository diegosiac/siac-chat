<script lang="ts">
	import * as Card from '@/ui/card';
	import { Label } from '@/ui/label';
	import { Input } from '@/ui/input';
	import { Button } from '@/ui/button';
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>Inicia con tu cuenta</Card.Title>
		<Card.Description>Completa el siguiente formulario para iniciar sesión.</Card.Description>
	</Card.Header>
	<Card.Content class=" space-y-2">
		<div class="space-y-1">
			<Label for="name">Nombre de usuario</Label>
			<Input id="name" value="Pedro Duarte" />
		</div>
		<div class="space-y-1">
			<Label for="username">Contraseña</Label>
			<Input id="username" value="@peduarte" />
		</div>
	</Card.Content>
	<Card.Footer class="justify-end">
		<Button>Iniciar sesión</Button>
	</Card.Footer>
</Card.Root>
