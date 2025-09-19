<script lang="ts">
	let { children } = $props()
</script>

<div class="flex w-full flex-row">
	<div class="flex w-60 flex-col gap-1">
		<a href="/admin/users">Users</a>
		<a href="/admin/feedback">Feedback</a>
		<a href="/admin/kv">Key Value</a>
		<a href="/admin/audit">Audit Logs</a>
	</div>

	<div class="flex-1">
		{@render children()}
	</div>
</div>
