<template>
  <div class="home">
    <div class="home-content">
      <h2>Trending projects</h2>
      <div class="dao-list">

        <div v-show ="item&&item.name&&item.name.length>0 " class="dao-item" @click="$router.push({path:'/Details',query:item})" v-for="(item,index) in homeArr" :key="index">
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
        <img style="margin: 10px auto" src="../imgs/no-data.webp" alt="" v-show="homeArr.length==0">
      </div>

<!--      <div class="more mangobox-button" @click="homeArr>maxCount?maxCount+=12:''">-->
<!--        More trending projects-->
<!--      </div>-->
    </div>
  </div>
</template>

<script>

import {getIpfs} from "../utils/ipfsApi"
import moment from "moment"
export default {
  name: 'HomeView',
  components: {},
  data() {
    return {
      moment:moment,
      maxCount:12,
      THomeArr:[]
    }
  },
  computed:{
    isConnected(){
      return this.$store.state.app.isConnected
    },
    homeArr(){
      return this.$store.state.app.homeArr
    }
  },
  watch:{
    isConnected(){
      this.getData()
    }
  },
  methods: {
    getMetaContent(id) {
      return this.$store.dispatch("MBProjects/getMetaContent", id)
    },
    getData(){
      // get IdArr => get hash => get json
      this.$store.dispatch("MBProjects/getProjectCount", this.$store.state.app.account).then(length => {
        let tempArr = []
        for(let i=1;i<=length;i++){
          this.getMetaContent(i).then(async res => {
            const jsonRes = await getIpfs(res)
            tempArr.push({
              id:i,
              ...jsonRes.data
            })
            if(i >= length){
              tempArr.sort((a,b)=>{
                return (parseInt(b.id) - parseInt(a.id))
              })
              this.$store.commit("app/SET_HOMEARR",tempArr)
              setTimeout(()=>{
                tempArr.sort((a,b)=>{
                  return (parseInt(b.id) - parseInt(a.id))
                })
                this.$store.commit("app/SET_HOMEARR",tempArr)
              },2000)
            }
          })
        }

      })
    }
  },
  created() {
    this.$store.dispatch("app/getWeb3").catch(()=>{
      this.$eventBus.$emit('message', {
        message: "Please Connect",
        type: "error"
      })
    }).then(()=>{
      this.getData()
    })
    setTimeout(()=>{
      if(this.homeArr.length<=0){
        this.getData()
      }
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
      flex-wrap: wrap;
      padding-bottom: 50px;
      .dao-item {
        display: flex;
        background: #fff;
        border-radius: 10px;
        box-shadow: 0px 5px 10px 5px rgba(0, 0, 0, 0.05);
        padding: 10px;
        cursor: pointer;
        margin-right: 20px;
        margin-top: 20px;
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
