<script>
import router from '@/router'
export default {
  data() {
    return {
      limit: 2,
      offset: 0,
      devices: [],
    }
  },
  methods: {
    async getDevices() {
      try {
        let res = await fetch(`/indexer/devices?limit=${this.limit}&offset=${this.offset}`, {
          method: 'GET',
        })
        switch (res.status) {
          case 200:
            break
          default:
            console.error(res)
            return
        }
        let data = await res.json()
        this.devices = data
      } catch (e) {
        console.error(e)
        return
      }
    },
    goToDevicePage(address) {
      router.push({
        name: 'device',
        params: {
          address,
        },
      })
    },
    left() {
      this.offset = this.offset - this.limit
      if (this.offset < 0) {
        this.offset = 0
        return
      }
      this.getDevices()
    },
    right() {
      if (this.devices.length === 0) {
        return
      }
      this.offset = this.offset + this.limit
      this.getDevices()
    },
  },
  mounted() {
    this.getDevices()
  },
}
</script>

<template>
  <h1>Devices</h1>
  <div>
    <table v-if="devices.length">
      <thead>
        <tr>
          <th>Address</th>
          <th>Data Type</th>
          <th>Location</th>
        </tr>
      </thead>
      <tbody>
        <tr
          class="mouse-pointer"
          @click="() => goToDevicePage(address)"
          v-for="{ address, device: { data_type, location } } in devices"
          :key="address"
        >
          <td>{{ address }}</td>
          <td>{{ data_type }}</td>
          <td>
            <a :href="`https://www.google.com/maps/place/${location}`" target="_blank">{{
              location
            }}</a>
          </td>
        </tr>
      </tbody>
    </table>
    <p v-else>There are no devices at the moment.</p>
    <div class="container pagination">
      <button class="left-button" type="button" @click="left">Left</button>
      <button class="right-button" type="button" @click="right">Right</button>
    </div>
  </div>
</template>
<style scoped>
.left-button {
  margin: 0 5px 0 0;
}

.right-button {
  margin: 0 0 0 5px;
}

.pagination {
  margin: 25px 0 0 0;
}

.pagination > button {
  padding: 0px 25px 5px 25px;
  width: 100%;
}
</style>
