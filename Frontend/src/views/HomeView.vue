<template>
  <div class="home">
    <div class="home-content">
      <h2>Trending projects</h2>
      <div class="dao-list">

        <div class="dao-item" @click="$router.push({path:'/Details',query:item})" v-for="(item,index) in HomeArr" :key="index">
          <div class="logo">
            <img :src="item.icon" alt=""/>
          </div>
          <div class="right">
            <div class="name">
              {{item.name}}
            </div>
            <div class="time">
              {{ moment(item.createTime).format('MMMM Do YYYY, h:mm:ss a')}}
            </div>
          </div>
        </div>
      </div>

      <div class="more mangobox-button" @click="upload">
        More trending projects
      </div>
    </div>
  </div>
</template>

<script>

import {getIpfs, uploadJson} from "../utils/ipfsApi"
import moment from "moment"
export default {
  name: 'HomeView',
  components: {},
  data() {
    return {
      HomeArr: [],
      moment:moment
    }
  },
  methods: {
    upload() {
      uploadJson({
        icon: 'https://avatars.githubusercontent.com/u/94897284?v=4',
        name: 'mangobox',
        createTime: new Date()
      }).then(res => {
        console.log(res.data.IpfsHash)
        getIpfs(res.data.IpfsHash)
      })
    },
    getMetaContent(id) {
      return this.$store.dispatch("MBProjects/getMetaContent", id)
    },
    getData(){
      // get IdArr => get hash => get json
      this.$store.dispatch("MBController/getOwnerProjects", this.$store.state.app.account).then(res => {
        console.log(res)
        res.forEach(id => {
          this.getMetaContent(id).then(async res => {
            const jsonRes = await getIpfs(res)
            this.HomeArr.push({
              ...jsonRes.data,
              id:id
            })
          })
        })

      })
    }
  },
  created() {
    this.getData()
    setTimeout(()=>{
      this.getData()
    },3000)

  }
}
</script>
<style lang="scss">
.home {
  background: #F8F8F8;
  overflow: hidden;

  .home-content {
    width: 1200px;
    margin: 20px auto;

    .dao-list {
      display: flex;

      .dao-item {
        display: flex;
        background: #fff;
        border-radius: 10px;
        box-shadow: 0px 5px 10px 5px rgba(0, 0, 0, 0.05);
        padding: 10px;
        cursor: pointer;
        margin-right: 20px;
        .logo {
          width: 70px;
          img{
            width: 90%;
          }
        }

        .right {
          width: 160px;

          .payments, .time {
            margin-top: 6px;
            color: #999;

            span {
              color: #FF7237;
            }
          }
        }
      }
    }

    .more {
      padding: 10px;
      width: 260px;
      margin: 15px auto;
    }
  }
}
</style>
