<template>
  <div class="mongobox-header" :style="'height:' + headerHeight + 'px;!important'">
    <div class="nav-box">
      <img @click="$router.push('/')" class="logo" src="../imgs/logo.webp" alt=""/>
      <ul>
        <li @click="$router.push('/projects')">
          <span class="w-font">Projects</span>
          <span class="re-active">Projects</span>
        </li>
        <li>
          <span class="w-font">FAQ</span>
          <span class="re-active">FAQ</span>
        </li>
        <li>
          <!--     Connect the wallet wrapper component     -->
          <polkaConnect/>
        </li>
      </ul>
    </div>
    <div class="home-content">
      <p>
        MangoBOX is a platform that empowers DAOs to fundraise from their community by setting up/managing an
        end-to-end fundraising process and issuing tokens to community members.
      </p>

      <button @click="$router.push({path:'/CreateProject'})" class="mangobox-button">Creat your project</button>

    </div>
  </div>
</template>

<script>
import polkaConnect from "@/components/polkaConnect";

export default {
  name: "mangobox-header",
  components: {
    polkaConnect
  },
  data() {
    return {}
  },
  computed: {
    headerHeight() {
      return this.$route.path === '/' ? 300 : 60
    }
  },

}
</script>

<style lang="scss" scoped>
.mongobox-header {
  background: url("../imgs/header_bg.webp");
  background-size: 100%;
  font-weight: bold;
  position: relative;
  width: 100%;
  overflow: hidden;

  .navbar-box-bg {
    width: 1920px;
    height: 60px;
    opacity: 0.1;
    background: #ffffff;
    position: absolute;
  }

  .nav-box {
    display: flex;
    height: 60px;
    justify-content: space-between;
    align-items: center;
    width: 1200px;
    margin: 0 auto;

    .logo {
      width: 140px;
      height: 26px;
    }

    ul {
      display: flex;
      height: 42px;

      li {
        height: 100%;
        line-height: 50px;
        position: relative;
        overflow: hidden;
        cursor: pointer;
        color: #111;
        margin: 0 30px;

        .w-font {
          position: relative;
          top: 0;
          left: 0;
          transition: 0s;
        }

        .re-active {
          position: absolute;
          top: 40px;
          left: 0;
          transition: 0s;
          background-image: linear-gradient(225deg, #54D500 0%, #2AAA00 100%);
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
        }

        &:hover {
          .re-active {
            top: 0 !important;
          }

          .w-font {
            top: -40px;
          }
        }
      }

    }
  }

  .home-content {
    width: 1200px;
    margin: 0 auto;

    a {
      text-decoration: none;
    }

    p {
      margin-top: 30px;
      width: 600px;
      text-align: justify;
    }

    button {
      margin-top: 30px;
      padding: 10px 20px;
    }
  }
}
</style>
