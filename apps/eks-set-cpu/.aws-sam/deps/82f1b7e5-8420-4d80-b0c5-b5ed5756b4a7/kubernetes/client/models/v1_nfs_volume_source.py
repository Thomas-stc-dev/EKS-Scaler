# coding: utf-8

"""
    Kubernetes

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)  # noqa: E501

    The version of the OpenAPI document: release-1.31
    Generated by: https://openapi-generator.tech
"""


import pprint
import re  # noqa: F401

import six

from kubernetes.client.configuration import Configuration


class V1NFSVolumeSource(object):
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
        'path': 'str',
        'read_only': 'bool',
        'server': 'str'
    }

    attribute_map = {
        'path': 'path',
        'read_only': 'readOnly',
        'server': 'server'
    }

    def __init__(self, path=None, read_only=None, server=None, local_vars_configuration=None):  # noqa: E501
        """V1NFSVolumeSource - a model defined in OpenAPI"""  # noqa: E501
        if local_vars_configuration is None:
            local_vars_configuration = Configuration()
        self.local_vars_configuration = local_vars_configuration

        self._path = None
        self._read_only = None
        self._server = None
        self.discriminator = None

        self.path = path
        if read_only is not None:
            self.read_only = read_only
        self.server = server

    @property
    def path(self):
        """Gets the path of this V1NFSVolumeSource.  # noqa: E501

        path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :return: The path of this V1NFSVolumeSource.  # noqa: E501
        :rtype: str
        """
        return self._path

    @path.setter
    def path(self, path):
        """Sets the path of this V1NFSVolumeSource.

        path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :param path: The path of this V1NFSVolumeSource.  # noqa: E501
        :type: str
        """
        if self.local_vars_configuration.client_side_validation and path is None:  # noqa: E501
            raise ValueError("Invalid value for `path`, must not be `None`")  # noqa: E501

        self._path = path

    @property
    def read_only(self):
        """Gets the read_only of this V1NFSVolumeSource.  # noqa: E501

        readOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :return: The read_only of this V1NFSVolumeSource.  # noqa: E501
        :rtype: bool
        """
        return self._read_only

    @read_only.setter
    def read_only(self, read_only):
        """Sets the read_only of this V1NFSVolumeSource.

        readOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :param read_only: The read_only of this V1NFSVolumeSource.  # noqa: E501
        :type: bool
        """

        self._read_only = read_only

    @property
    def server(self):
        """Gets the server of this V1NFSVolumeSource.  # noqa: E501

        server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :return: The server of this V1NFSVolumeSource.  # noqa: E501
        :rtype: str
        """
        return self._server

    @server.setter
    def server(self, server):
        """Sets the server of this V1NFSVolumeSource.

        server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs  # noqa: E501

        :param server: The server of this V1NFSVolumeSource.  # noqa: E501
        :type: str
        """
        if self.local_vars_configuration.client_side_validation and server is None:  # noqa: E501
            raise ValueError("Invalid value for `server`, must not be `None`")  # noqa: E501

        self._server = server

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
        if not isinstance(other, V1NFSVolumeSource):
            return False

        return self.to_dict() == other.to_dict()

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        if not isinstance(other, V1NFSVolumeSource):
            return True

        return self.to_dict() != other.to_dict()
