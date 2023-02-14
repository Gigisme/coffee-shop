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

<div class="flex items-center justify-center p-12">
  <div class="mx-auto w-full max-w-[550px] bg-sky-900 p-4 rounded">
    <form on:submit|preventDefault={handle}>
      <div class="mb-5">
        <label
          for="name"
          class="mb-3 block text-base font-medium text-[#07074D] dark:text-sky-50"
        >
          Name
        </label>
        <input
          id="name"
          type="text"
          required
          bind:value={itemName}
          class="w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md"
        />
      </div>
      <div class="mb-5">
        <label
          for="desc"
          class="mb-3 block text-base font-medium text-[#07074D] dark:text-sky-50"
        >
          Description
        </label>
        <textarea
          id="desc"
          bind:value={description}
          class="w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md"
        />
      </div>
      <div class="mb-5">
        <label
          for="price"
          class="mb-3 block text-base font-medium text-[#07074D] dark:text-sky-50"
        >
          Price
        </label>
        <input
          id="price"
          required
          bind:value={price}
          class="w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md"
        />
      </div>
      <div class="mb-5">
        <label
          for="weight"
          class="mb-3 block text-base font-medium text-[#07074D] dark:text-sky-50"
        >
          Weight
        </label>
        <input
          id="weight"
          required
          bind:value={weight}
          class="w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md"
        />
      </div>
      <div class="mb-5">
        <label
          for="rating"
          class="mb-3 block text-base font-medium text-[#07074D] dark:text-sky-50"
        >
          Rating
        </label>
        <input
          id="rating"
          bind:value={rating}
          class="w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md"
        />
      </div>
      <div>
        <button
          class="hover:shadow-form rounded-md bg-[#6A64F1] py-3 px-8 text-base font-semibold text-white outline-none"
        >
          Submit
        </button>
      </div>
    </form>
  </div>
</div>
