import 'package:app_mobile/models/device.dart';
import 'package:flutter/material.dart';
import 'package:app_mobile/shared/constants.dart';

class InactiveDevice extends StatelessWidget {
  final FirebaseDevice firebaseDevice;

  InactiveDevice(this.firebaseDevice);

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
        constraints: BoxConstraints(minHeight: CardMinHeight),
        child: Card(
            child: InkWell(
          splashColor: Colors.blue.withAlpha(30),
          onTap: () {
            print('Card tapped.');
          },
          child: Container(
              child: Column(
            children: [
              SizedBox(
                height: 5,
              ),
              Text(firebaseDevice.type.toUpperCase(),
                  style: TextStyle(fontSize: 24)),
              SizedBox(
                height: 5,
              ),
              Column(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.warning,
                      size: 48,
                    ),
                    Text("Device is not active",
                        style: TextStyle(fontSize: 16)),
                    GestureDetector(
                      child: Text(
                        "Need help? Click here",
                        style: TextStyle(fontSize: 12, color: LayoutBlueColor1),
                      ),
                      onTap: () {
                        print("Tap need help");
                      },
                    )
                  ]),
            ],
          )),
        )));
  }
}