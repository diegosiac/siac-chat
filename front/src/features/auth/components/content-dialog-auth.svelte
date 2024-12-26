<script lang="ts">
	import * as Dialog from '@/ui/dialog';
	import * as Tabs from '@/ui/tabs';
	import { LoginForm, RegisterForm } from '@/features/auth/components';
</script>

<Dialog.Content class="h-[550px] flex flex-col">
	<Dialog.Header>
		<Dialog.Title>Bienvenido a SiacChat</Dialog.Title>
		<Dialog.Description>
			Por favor, inicia sesión o regístrate para comenzar a chatear.
		</Dialog.Description>
	</Dialog.Header>

	<Tabs.Root value="login" class="h-full">
		<Tabs.List class="grid w-full grid-cols-2">
			<Tabs.Trigger value="login">Iniciar Sesión</Tabs.Trigger>
			<Tabs.Trigger value="signup">Registrarse</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="login" class="h-full">
			<LoginForm />
		</Tabs.Content>
		<Tabs.Content value="signup">
			<RegisterForm />
		</Tabs.Content>
	</Tabs.Root>
</Dialog.Content>
