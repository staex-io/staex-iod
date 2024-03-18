<script>
import router from '@/router'
export default {
  data() {
    return {
      limit: 2,
      offset: 0,
      devices: [],
      newDevicesLength: 0,
      filterField: '',
      filterCondition: '',
      filterValue: '',
      filter: null,
    }
  },
  methods: {
    async getDevices() {
      try {
        let url = `/indexer/devices?limit=${this.limit}&offset=${this.offset}`
        if (this.filter !== null) {
          if (this.filter.field === 'address') {
            url = `${url}&address=${this.filter.value}`
          } else {
            url = `${url}&filters[0][field]=${this.filter.field}&filters[0][condition]=${this.filter.condition}&filters[0][value]=${this.filter.value}`
          }
        }
        let res = await fetch(url, {
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
        this.newDevicesLength = data.length
        if (this.offset !== 0 && this.newDevicesLength === 0) return
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
      const oldOffset = this.offset
      this.offset = this.offset + this.limit
      this.getDevices().then(() => {
        if (this.newDevicesLength === 0) this.offset = oldOffset
      })
    },
    clearFilter() {
      this.offset = 0
      this.filter = null
      this.filterField = ''
      this.filterCondition = ''
      this.filterValue = ''
      this.getDevices()
    },
    handleFilter() {
      this.offset = 0
      if (this.filterField === '' || this.filterCondition === '' || this.filterValue === '') {
        return
      }
      this.filter = {
        field: this.filterField,
        condition: this.filterCondition,
        value: this.filterValue,
      }
      this.getDevices()
    },
  },
  mounted() {
    this.getDevices()
  },
}
</script>

<template>
  <div class="container filter">
    <div class="item">
      <form class="form-container" @submit.prevent="handleFilter">
        <select id="filterField" v-model="filterField" style="margin-right: 5px">
          <option disabled value="" selected>Select field</option>
          <option
            v-for="key in ['address', 'data_type', 'price_access', 'price_pin']"
            :key="key"
            :value="key"
          >
            {{ key }}
          </option>
        </select>
        <select id="filterCondition" v-model="filterCondition" style="margin-right: 5px">
          <option disabled value="" selected>Select condition</option>
          <option v-for="key in ['=', '<', '>']" :key="key" :value="key">
            {{ key }}
          </option>
        </select>
        <input
          type="text"
          name="filterValue"
          id="filterValue"
          placeholder="Field value"
          v-model="filterValue"
        />
        <button type="button" @click="handleFilter" style="margin-right: 5px">Apply</button>
        <button type="button" @click="clearFilter">Clear</button>
      </form>
    </div>
  </div>
  <h1>Devices</h1>
  <div class="h-scroll-container">
    <table v-if="devices.length">
      <thead>
        <tr>
          <th>Address</th>
          <th>Data Type</th>
          <th>Location</th>
          <th>Price access</th>
          <th>Price pin</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="{ address, device: { data_type, location, price_access, price_pin } } in devices"
          :key="address"
        >
          <td class="mouse-pointer" @click="() => goToDevicePage(address)">
            {{ address.slice(0, 4) + '..' + address.slice(44, 48) }}
          </td>
          <td>{{ data_type.slice(0, 10) }}</td>
          <td>
            <a :href="`https://www.google.com/maps/place/${location}`" target="_blank">{{
              location
            }}</a>
          </td>
          <td>
            {{ Intl.NumberFormat('en-IN', { maximumSignificantDigits: 10 }).format(price_access) }}
          </td>
          <td>
            {{ Intl.NumberFormat('en-IN', { maximumSignificantDigits: 10 }).format(price_pin) }}
          </td>
        </tr>
      </tbody>
    </table>
    <p v-else>There are no devices at the moment.</p>
  </div>
  <div class="container pagination">
    <button class="left-button" type="button" @click="left">Left</button>
    <button class="right-button" type="button" @click="right">Right</button>
  </div>
</template>

<style scoped>
.item {
  display: block;
}

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
  padding: 5px 25px 5px 25px;
  width: 100%;
}

.filter {
  margin: 25px 0 25px 0;
}
</style>
