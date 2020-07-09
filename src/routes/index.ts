import express from 'express';
import apiRouter from './api';
import alarmclockRouter from './alarmclock';
import watermixerRouter from './watermixer';
import { saveRequestToDb } from '../firebase';
import { getIpStr, getCountryStr } from '../helpers';

const router = express.Router();

const noLogUrl = ['/api/login'];

// create history on any POST request
router.use(
  (req: express.Request, res: express.Response, next: express.NextFunction) => {
    if (req.method !== 'POST') {
      next();
      return;
    }
    console.log('Saving to firebase DB');
    if (!noLogUrl.includes(req.url)) {
      saveRequestToDb({
        user: String(req.get('username')),
        requestPath: req.url,
        unixTime: new Date().getTime(),
        ip: getIpStr(req),
        userAgent: String(req.get('user-agent')),
        country: getCountryStr(req),
      });
    }
    next();
  },
);

router.use('/api', apiRouter);
router.use('/alarmclock', alarmclockRouter);
router.use('/watermixer', watermixerRouter);

router.get('/', (req, res): void => {
  res.send('Hello from API server');
});

export default router;
