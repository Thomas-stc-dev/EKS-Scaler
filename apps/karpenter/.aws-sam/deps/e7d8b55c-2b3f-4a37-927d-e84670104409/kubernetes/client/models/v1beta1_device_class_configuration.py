# coding: utf-8

"""
    Kubernetes

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)  # noqa: E501

    The version of the OpenAPI document: release-1.32
    Generated by: https://openapi-generator.tech
"""


import pprint
import re  # noqa: F401

import six

from kubernetes.client.configuration import Configuration


class V1beta1DeviceClassConfiguration(object):
    """NOTE: This class is auto generated by OpenAPI Generator.
    Ref: https://openapi-generator.tech

    Do not edit the class manually.
    """

    """
    Attributes:
      openapi_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    openapi_types = {
        'opaque': 'V1beta1OpaqueDeviceConfiguration'
    }

    attribute_map = {
        'opaque': 'opaque'
    }

    def __init__(self, opaque=None, local_vars_configuration=None):  # noqa: E501
        """V1beta1DeviceClassConfiguration - a model defined in OpenAPI"""  # noqa: E501
        if local_vars_configuration is None:
            local_vars_configuration = Configuration()
        self.local_vars_configuration = local_vars_configuration

        self._opaque = None
        self.discriminator = None

        if opaque is not None:
            self.opaque = opaque

    @property
    def opaque(self):
        """Gets the opaque of this V1beta1DeviceClassConfiguration.  # noqa: E501


        :return: The opaque of this V1beta1DeviceClassConfiguration.  # noqa: E501
        :rtype: V1beta1OpaqueDeviceConfiguration
        """
        return self._opaque

    @opaque.setter
    def opaque(self, opaque):
        """Sets the opaque of this V1beta1DeviceClassConfiguration.


        :param opaque: The opaque of this V1beta1DeviceClassConfiguration.  # noqa: E501
        :type: V1beta1OpaqueDeviceConfiguration
        """

        self._opaque = opaque

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.openapi_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, V1beta1DeviceClassConfiguration):
            return False

        return self.to_dict() == other.to_dict()

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        if not isinstance(other, V1beta1DeviceClassConfiguration):
            return True

        return self.to_dict() != other.to_dict()
