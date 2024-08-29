
/// <reference path='./.sst/platform/config.d.ts' />

const domain = "discitapp.com"

export default $config({
  app(input) {
    return {
      name: 'discit-backend',
      removal: 'remove',
      home: 'aws',
      providers: {
        aws: { region: 'us-east-1' }
      },
      stage: input?.stage
    };
  },
  async run() {
    const { stage } = $app;
    const environment = {
      STAGE: stage,
      AWS_REGION: 'us-east-1',
      LOG_LEVEL: process.env.LOG_LEVEL,
      AWS_ACCESS_KEY_ID: process.env.AWS_ACCESS_KEY_ID,
      AWS_SECRET_ACCESS_KEY: process.env.AWS_SECRET_ACCESS_KEY,
      MONGO_URI: process.env.MONGO_URI,
    }

    const cron = new sst.aws.Cron('nightly-backup', {
      schedule: 'cron(0 0 * * ? *)',
      job: {
        logging: { retention: '1 week', format: 'json' },
        environment,
        architecture: 'arm64',
        runtime: 'provided.al2023',
        handler: 'bootstrap',
        bundle: 'target/lambda/nightly-backup',
      }
    });

    const api = new sst.aws.Function('api', {
      handler: 'bootstrap',
      bundle: 'target/lambda/api',
      memory: '1024 MB',
      timeout: '10 minutes',
      architecture: "arm64",
      url: { cors: true, allowCredentials: true },
      logging: { retention: '1 week', format: 'json' },
      environment,
      link: []
    });

    return {
      url: api.url,
    }
  },
});
