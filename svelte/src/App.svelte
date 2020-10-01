<script>
	let uuid = '';
	let username = '';
	let uuid_is_real = false;
	let error_text = '';

	async function check_account() {
		await fetch(`https://api.minetools.eu/uuid/${username}`)
				.then(r => r.json())
				.then(data => {
					if (data.status !== "OK") {
						error_text = "Player not found. You may have made a typo, or Mojang's servers are having issues."
					} else {
						uuid = data.id;
						uuid_is_real = true;
					}
				});
	}

	function edit_account() {
		uuid_is_real = false;
	}
</script>

<h1>Link Your Minecraft Account</h1>
<p>{error_text}</p>
<form method="post">
	<label for="username">Minecraft Username</label>
	<input type="text" bind:value={username} id="username" name="username" readonly={uuid_is_real}>
	<input type="text" id="uuid" name="uuid" value={uuid} hidden readonly>
	{#if uuid_is_real}
		<button on:click={edit_account}>✏ Change Account</button>
		<input type="submit" value="✔ Link Account">
	{:else}
		<button type="button" on:click={check_account}>🔁 Check Account</button>
	{/if}
</form>

{#if uuid_is_real}
	<p>Minecraft account found! If this is you, press the Link Account button to confirm.</p>
	<img src="https://crafatar.com/renders/body/{uuid}" alt="found minecraft skin">
{/if}
