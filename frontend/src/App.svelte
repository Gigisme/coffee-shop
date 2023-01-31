<script>
  let itemName;
  let description;
  let price;
  let weight;
  let rating;

  const handle = async () => {
    let response = await fetch("http://127.0.0.1:8080/", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        name: itemName,
        description: description,
        price: price,
        weight: weight,
        rating: rating,
      }),
    });
    if (response.ok) {
    } else {
      console.log(response.status);
    }
  };
</script>

<main>
  <form on:submit|preventDefault={handle}>
    <label>
      Name:
      <input type="text" bind:value={itemName} />
    </label>
    <label>
      Desc:
      <input type="text" bind:value={description} />
    </label>
    <input type="number" bind:value={price} />
    <input type="number" bind:value={weight} />
    <input type="number" bind:value={rating} />
    <button>Add item</button>
  </form>
</main>

<style>
</style>
