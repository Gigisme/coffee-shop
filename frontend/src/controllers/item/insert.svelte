<script lang="ts">
  let itemName: string;
  let description: string;
  let price: number;
  let weight: number;
  let rating: number;

  const handle = async () => {
    let response = await fetch("http://127.0.0.1:8080/insertItem", {
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

<form
  on:submit|preventDefault={handle}
  class="bg-grey-50 rounded shadow-md px-8 dark:bg-sky-900"
>
  <div>
    <label>
      Name:
      <input type="text" bind:value={itemName} />
    </label>
  </div>
  <div>
    <label>
      Desc:
      <input type="text" bind:value={description} />
    </label>
  </div>
  <div>
    <label>
      Price:
      <input type="number" bind:value={price} />
    </label>
  </div>
  <div>
    <label>
      Weight:
      <input type="number" bind:value={weight} />
    </label>
  </div>
  <div>
    <label>
      Rating:
      <input type="number" bind:value={rating} />
    </label>
  </div>
  <button>Add item</button>
</form>
